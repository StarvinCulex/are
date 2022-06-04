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
        if CHUNK_WIDTH == 1 && CHUNK_HEIGHT == 1 {
            let elements: &[Element] = unsafe { std::mem::transmute(self.elements.as_slice()) };
            state.serialize_field("elems", elements)
        } else {
            let elements: Vec<_> = self.as_area().scan().map(|(_, e)| e).collect();
            state.serialize_field("elems", &elements)
        }?;
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
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field { Cols, Elems }

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
                let mut cols: Option<u32> = None;
                let mut elems: Option<Vec<Element>> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Cols => {
                            if unlikely(cols.is_some()) {
                                return Err(Error::duplicate_field("cols"));
                            }
                            cols = Some(map.next_value()?);
                        }
                        Field::Elems => {
                            if unlikely(elems.is_some()) {
                                return Err(Error::duplicate_field("elems"));
                            }
                            elems = Some(map.next_value()?);
                        }
                    }
                }
                let cols = cols.ok_or_else(|| Error::missing_field("cols"))? as usize;
                let elems = elems.ok_or_else(|| Error::missing_field("elems"))?;
                if unlikely(cols == 0) {
                    return Err(Error::custom("`cols` must greater than 0"));
                }
                if unlikely(elems.len() % cols != 0) {
                    return Err(Error::custom("`cols` cannot divide length of `elems`"));
                }
                let rows = elems.len() / cols;
                if unlikely(rows == 0) {
                    return Err(Error::custom("`rows` must greater than 0"));
                }
                let size = Coord(cols, rows);
                if CHUNK_WIDTH == 1 && CHUNK_HEIGHT == 1 {
                    let origin_matrix = Matrix::with_data(size, elems).unwrap();
                    // no actual transmute occurs here
                    return Ok(unsafe { std::mem::transmute(origin_matrix) });
                }
                // with_iter() introduces extra check, this is faster
                let mut matrix = unsafe { Self::Value::uninit(size) };
                for (src, (_, dest)) in elems.into_iter().zip(matrix.as_area_mut().scan()) {
                    unsafe { std::ptr::write(dest, src) };
                }
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
    // JSON object is unordered: test of reversing the order of elems and cols
    let target2: Matrix<String, 7, 9> = serde_json::from_str("{\"elems\":[\"(0, 0)\"],\"cols\":1}").unwrap();
}
