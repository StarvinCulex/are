use std::fmt::Formatter;

use serde::de::{Error, SeqAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl<Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> Serialize
    for Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>
where
    Element: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Matrix", 3)?;
        state.serialize_field("cols", &(self.size.0 as u32))?;
        let elements: Vec<_> = self.as_area().scan().map(|(_, e)| e).collect();
        state.serialize_field("elems", &elements)?;
        state.end()
    }
}

impl<'de, Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> Deserialize<'de>
    for Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>
where
    Element: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MatrixSizeVisitor<Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> {
            phantom: std::marker::PhantomData<Element>,
        }
        impl<'de, Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> Visitor<'de>
            for MatrixSizeVisitor<Element, CHUNK_WIDTH, CHUNK_HEIGHT>
        where
            Element: Deserialize<'de>,
        {
            type Value = Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>;
            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("Matrix")
            }
            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: serde::de::MapAccess<'de>,
            {
                let cols_entry: (String, u32) = map
                    .next_entry()?
                    .ok_or_else(|| Error::custom("need param `cols`"))?;
                let cols = cols_entry.1 as usize;
                if cols == 0 {
                    return Err(Error::custom("`cols` must greater than 0"));
                }

                let elems_entry: (String, Vec<Element>) = map
                    .next_entry()?
                    .ok_or_else(|| Error::custom("need param `elems`"))?;
                let elems = elems_entry.1;
                if elems.len() % cols != 0 {
                    return Err(Error::custom("`cols` cannot divide length of `elems`"));
                }
                let rows = elems.len() / cols;
                if rows == 0 {
                    return Err(Error::custom("`rows` must greater than 0"));
                }
                let origin_matrix = Matrix::with_data(Coord(cols, rows), elems).unwrap();
                let matrix = if CHUNK_WIDTH == 1 && CHUNK_HEIGHT == 1 {
                    unsafe { std::mem::transmute(origin_matrix) }
                } else {
                    Matrix::<Element, CHUNK_WIDTH, CHUNK_HEIGHT>::with_iter(
                        Coord(cols, rows),
                        origin_matrix.into_iter(),
                    )
                    .unwrap()
                };
                Ok(matrix)
            }
        }
        deserializer.deserialize_struct(
            "Matrix",
            &["cols", "elems"],
            MatrixSizeVisitor::<Element, CHUNK_WIDTH, CHUNK_HEIGHT> {
                phantom: Default::default(),
            },
        )
    }
}

#[cfg(test)]
#[test]
fn test_serde() {
    let origin = Matrix::<String, 5, 2>::with_ctor(Coord(7usize, 13usize), |p| p.to_string());
    let ser_json = serde_json::to_string(&origin).unwrap();
    let target: Matrix<String, 3, 1> = serde_json::from_str(ser_json.as_str()).unwrap();
    assert_eq!(origin.size(), target.size());
    for j in 0..origin.size().1 {
        for i in 0..origin.size().0 {
            let p = Coord(i, j);
            assert_eq!(origin[p], target[p]);
        }
    }
}
