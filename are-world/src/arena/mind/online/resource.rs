use crate::SWord;

pub const ELEMENT_DISPLAY_WIDTH: usize = 5;
pub const ELEMENT_DISPLAY_HEIGHT: usize = 1;

pub fn resource(
    name: &String,
    phase: usize,
) -> [[char; ELEMENT_DISPLAY_HEIGHT]; ELEMENT_DISPLAY_WIDTH] {
    let s = name.to_string();
    let mut name = s.chars();
    let mut texture = [[' '], [' '], [' '], [' '], [' ']];
    for i in 0..5 {
        if let Some(ch) = name.next() {
            texture[i][0] = ch;
        } else {
            break;
        }
    }
    texture
}
