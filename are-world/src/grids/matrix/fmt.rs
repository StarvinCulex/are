//! by *StarvinCulex @2021/11/27*
use super::*;

const DELIM: &str = ",";
const LINE_DELIM: &str = "\n";
const NONE: &str = "";
const EMPTY_STR: &str = "\"\"";

const HEAD_MATRIX: &str = "M";

impl<Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> std::fmt::Display
    for Matrix<Element, CHUNK_WIDTH, CHUNK_HEIGHT>
where
    Element: std::string::ToString,
{
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.iter().fmt_with(f, HEAD_MATRIX)
    }
}

impl<'m, Element, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> std::fmt::Display
    for Area<'m, Element, CHUNK_WIDTH, CHUNK_HEIGHT>
where
    Element: std::string::ToString,
{
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.scan().fmt_with(f, "Area")
    }
}

impl<'m, Element, Access, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> std::fmt::Display
    for Iterator<'m, Element, Access, CHUNK_WIDTH, CHUNK_HEIGHT>
where
    Access: Accessor<CHUNK_WIDTH, CHUNK_HEIGHT>,
    Element: std::string::ToString,
{
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_with(f, self.accessor.r#type())
    }
}

impl<'m, Element, Access, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize>
    Iterator<'m, Element, Access, CHUNK_WIDTH, CHUNK_HEIGHT>
where
    Access: Accessor<CHUNK_WIDTH, CHUNK_HEIGHT>,
    Element: std::string::ToString,
{
    fn fmt_with(&self, f: &mut std::fmt::Formatter<'_>, head: &str) -> std::fmt::Result {
        let start = self.accessor.super_area().from();
        let size = measure_area(*self.matrix.size(), self.accessor.super_area());
        let sheet_size = Coord(size.0 as usize + 1, size.1 as usize + 1);

        let constructor = |display_offset: Coord<isize>| {
            if display_offset.0 == 0 {
                if display_offset.1 == 0 {
                    head.to_string()
                } else {
                    let Coord(_, display_index) =
                        self.matrix.normalize(start + display_offset - Coord(1, 1));
                    print_index(display_index)
                }
            } else if display_offset.1 == 0 {
                let Coord(display_index, _) =
                    self.matrix.normalize(start + display_offset - Coord(1, 1));
                print_index(display_index)
            } else {
                let index = display_offset - Coord(1, 1);
                let pos = start + index;
                if self.accessor.contains(pos) {
                    print_element(&self.matrix[pos])
                } else {
                    NONE.into()
                }
            }
        };

        let sheet = Matrix::<String, 1, 1>::with_ctor(sheet_size, constructor);
        let mut widths = vec![0usize; sheet_size.0];
        for (p, s) in sheet.iter() {
            let col = p.0 as usize;
            let len = string_width(s);
            widths[col] = std::cmp::max(widths[col], len);
        }

        for j in 0..size.1 + 1 {
            for i in 0..size.0 + 1 {
                if i != 0 {
                    write!(f, "{}", DELIM)?;
                } else if j != 0 {
                    write!(f, "{}", LINE_DELIM)?;
                }
                write!(
                    f,
                    "{value:^width$}",
                    value = sheet[Coord(i, j)],
                    width = widths[i as usize],
                )?;
            }
        }

        Ok(())
    }
}

#[inline]
fn print_element<Element: std::string::ToString>(element: &Element) -> String {
    const QT: u8 = b'\"';
    const PQT: [u8; 2] = *b"\"\"";
    const CR: u8 = b'\r';
    const PCR: [u8; 2] = *b"\\r";
    const LF: u8 = b'\n';
    const PLF: [u8; 2] = *b"\\n";
    const TB: u8 = b'\t';
    const PTB: [u8; 2] = *b"\\t";
    const COMMA: u8 = b',';
    const SPACE: u8 = b' ';

    let raw = element.to_string();

    if raw.is_empty() {
        return EMPTY_STR.to_string();
    }

    let mut buf = vec![];
    let mut with_quotes = false;

    for ch in raw.as_bytes() {
        match *ch {
            QT => {
                if !with_quotes {
                    buf.insert(0, QT);
                    with_quotes = true;
                }
                buf.extend(PQT);
            }
            CR => {
                if !with_quotes {
                    buf.insert(0, QT);
                    with_quotes = true;
                }
                buf.extend(PCR);
            }
            LF => {
                if !with_quotes {
                    buf.insert(0, QT);
                    with_quotes = true;
                }
                buf.extend(PLF);
            }
            TB => {
                if !with_quotes {
                    buf.insert(0, QT);
                    with_quotes = true;
                }
                buf.extend(PTB);
            }
            SPACE | COMMA => {
                if !with_quotes {
                    buf.insert(0, QT);
                    with_quotes = true;
                }
                buf.push(*ch);
            }
            _ => buf.push(*ch),
        }
    }

    if with_quotes {
        buf.push(QT);
    }

    String::from_utf8(buf).unwrap()
}

#[inline]
fn print_index(num: isize) -> String {
    num.to_string()
}

#[inline]
fn string_width(s: &str) -> usize {
    s.len()
}
