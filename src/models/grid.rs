//make sure these values match the CSS!!
// pub const FONT_SIZE_PX: u32 = 32;
pub const CELL_WIDTH: f64 = 19.2; //TODO formula (FONT_SIZE_PX * 3) / 5 ?
pub const LINE_HEIGHT: f64 = 44.0;

pub type Coord = (usize, usize);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct CellStyle {
    pub background: Color,
    pub foreground: Color,
    pub bold: bool,
    pub italic: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cell {
    pub char: char,
    pub style: CellStyle,
    pub coord: Coord
}

#[allow(dead_code)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Color {
    NONE,
    BLACK,
    WHITE,
    RED,
    GREEN,
    BLUE,
    ORANGE,
}

impl Color {
    pub fn value(&self) -> String {
        match *self {
            Color::NONE => String::new(),
            Color::BLACK => "#000000".to_string(),
            Color::WHITE => "#ffffff".to_string(),
            Color::RED => "#ff0000".to_string(),
            Color::GREEN => "#00ff00".to_string(),
            Color::BLUE => "#0000ff".to_string(),
            Color::ORANGE => "#ffa500".to_string(),
        }
    }
}

#[allow(dead_code)]
pub enum TextAlign {
    LEFT,
    CENTER,
    RIGHT
}

pub fn push_str_to_grid(grid: &mut Vec<Cell>, coord: Coord, str: &str, style: CellStyle, align: TextAlign) {
    let new_coord: Coord = match align {
        TextAlign::LEFT => (coord.0, coord.1),
        TextAlign::CENTER => (coord.0.saturating_sub(str.len() / 2), coord.1),
        TextAlign::RIGHT => (coord.0.saturating_sub(str.len()-1), coord.1),
    };

    for (i, char) in str.chars().enumerate() {
        grid.push(Cell {
            char,
            style,
            coord: (new_coord.0 + i, new_coord.1)
        });
    }
}

pub fn push_char_to_grid(grid: &mut Vec<Cell>, coord: Coord, char: char, style: CellStyle) {
    grid.push(Cell {
        char,
        style,
        coord
    });
}
