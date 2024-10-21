use std::fmt;
use std::collections::HashMap;

//make sure these values match the CSS!!
// pub const FONT_SIZE_PX: u32 = 32;
pub const CELL_WIDTH: f64 = 19.2; //TODO formula (FONT_SIZE_PX * 3) / 5 ?
pub const LINE_HEIGHT: f64 = 44.0;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CellStyle {
    pub background: Color,
    pub foreground: Color,
    pub bold: bool,
    pub italic: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub char: char,
    pub style: CellStyle
}

impl Cell {
    pub fn empty() -> Cell {
        Cell {
            char: ' ',
            style: CellStyle {
                background: Color::NONE,
                foreground: Color::WHITE,
                bold: false,
                italic: false,
            }
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Color {
    NONE,
    BLACK,
    WHITE,
    RED,
    GREEN,
    BLUE,
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
        }
    }
}

pub type Grid = HashMap<(usize, usize), Cell>;
pub type GridCoord = (usize, usize);
pub enum TextAlign {
    LEFT,
    CENTER,
    RIGHT
}

pub fn insert_string(grid: &mut Grid, coord: GridCoord, str: &str, style: CellStyle, align: TextAlign) {
    let new_coord: GridCoord = match align {
        TextAlign::LEFT => (coord.0, coord.1),
        TextAlign::CENTER => (coord.0.saturating_sub(str.len() / 2), coord.1),
        TextAlign::RIGHT => (coord.0.saturating_sub(str.len()-1), coord.1),
    };

    for (i, char) in str.chars().enumerate() {
        grid.insert((new_coord.0 + i, new_coord.1), Cell {
            char,
            style
        });
    }
}
