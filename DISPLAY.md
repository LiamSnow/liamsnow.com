# Display Protocol

Maintains a buffer that is a grid of cells (WxH).
 - Grid must automatically resize on screen adjustments
 - Grid must adjust display to fit entire screen

In order to reduce size of the buffer there is a lookup table
of colors (255 max), which is referenced by the cell model.
 - each cell is `26` bits (would be `52` with full color)

```rust
pub struct Cell {
    pub char: char,
    pub background: u8, //index -> color table
    pub foreground: u8, //index -> color table
    pub bold: bool,
    pub italic: bool,
}
```
