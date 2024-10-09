//make sure these values match the CSS!!
// pub const FONT_SIZE_PX: u32 = 32;
pub const CELL_WIDTH: f64 = 19.2; //TODO formula (FONT_SIZE_PX * 3) / 5 ?
pub const LINE_HEIGHT: f64 = 44.0;

#[derive(Clone)]
pub struct Cell {
    pub char: char,
    pub background: u8, //index -> color table
    pub foreground: u8, //index -> color table
    pub bold: bool,
    pub italic: bool,
}

impl Cell {
    pub fn empty() -> Cell {
        Cell {
            char: ' ',
            background: 0,
            foreground: 0,
            bold: false,
            italic: false
        }
    }
}

