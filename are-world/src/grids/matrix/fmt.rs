// by *StarvinCulex @2021/11/27*

const DELIM: &str = ",";
const QUOTE: char = '"';
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

impl<'m, Element, Access, const CHUNK_WIDTH: usize, const CHUNK_HEIGHT: usize> std::fmt::Display
    for Iterator<'m, Element, Access, CHUNK_WIDTH, CHUNK_HEIGHT>
where
    Access: Accessor<CHUNK_WIDTH, CHUNK_HEIGHT>,
    Element: std::string::ToString,
{
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

        let constructor = |opt_index: Option<Coord<isize>>| {
            if let Some(display_index) = opt_index {
                if display_index.0 == 0 {
                    if display_index.1 == 0 {
                        head.to_string()
                    } else {
                        print_index(display_index.1 - 1)
                    }
                } else if display_index.1 == 0 {
                    print_index(display_index.0 - 1)
                } else {
                    let index = display_index - Coord(1, 1);
                    let pos = start + index;
                    if self.accessor.contains(pos) {
                        print_element(&self.matrix[pos])
                    } else {
                        NONE.into()
                    }
                }
            } else {
                String::new()
            }
        };

        let grids = Matrix::<String, 1, 1>::with_ctor(&sheet_size, constructor);
        let mut widths = vec![0usize; sheet_size.0];
        for (p, s) in grids.iter() {
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
                    value = grids[Coord(i, j)],
                    width = widths[i as usize],
                )?;
            }
        }

        Ok(())
    }
}

#[inline]
fn print_element<Element: std::string::ToString>(element: &Element) -> String {
    const QT: u8 = QUOTE as u8;
    const PQT: [u8; 2] = ['"' as u8, '"' as u8];
    const CR: u8 = '\r' as u8;
    const PCR: [u8; 2] = ['\\' as u8, 'r' as u8];
    const LF: u8 = '\n' as u8;
    const PLF: [u8; 2] = ['\\' as u8, 'n' as u8];
    const TB: u8 = '\t' as u8;
    const PTB: [u8; 2] = ['\\' as u8, 't' as u8];
    const COMMA: u8 = ',' as u8;
    const SPACE: u8 = ' ' as u8;

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
fn string_width(s: &String) -> usize {
    s.len()
}
