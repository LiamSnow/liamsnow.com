pub const CELL_WIDTH: u32 = 24;
pub const CELL_HEIGHT: u32 = 24;

pub struct Cell {
    pub char: char,
    pub background: u8, //index -> color table
    pub foreground: u8, //index -> color table
    pub bold: bool,
    pub italic: bool,
}

pub struct GridInfo {
    pub x_cells: u32,
    pub y_cells: u32,
    pub x_padding: u32,
    pub y_padding: u32
}
