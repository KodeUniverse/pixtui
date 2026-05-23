use crate::pixels::{Pixel, PixelGrid};
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::style::Color;
use ratatui::widgets::Widget;

pub struct PixelCanvas {
    pub grid: PixelGrid,
    pub x: u16,
    pub y: u16,
}

impl PixelCanvas {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            grid: PixelGrid::new(width, height),
            x: 0,
            y: 0,
        }
    }
    pub fn move_marker_up(&mut self, by: u16) {
        self.y -= by;
    }
    pub fn move_state_down(&mut self, by: u16) {
        self.y += by;
    }
    pub fn move_state_right(&mut self, by: u16) {
        self.x += by;
    }
    pub fn move_state_left(&mut self, by: u16) {
        self.x -= by;
    }
}

impl Default for PixelCanvas {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            grid: PixelGrid::new(64, 64),
        }
    }
}

impl Widget for &PixelCanvas {
    fn render(self, _area: Rect, buf: &mut Buffer) {
        let table_height = ((self.grid.height + 1) / 2) as usize;
        let table_width = self.grid.width as usize;

        for row in 0..table_height {
            let (row_upper, row_lower) = (row * 2, row * 2 + 1);
            for col in 0..table_width {
                let upper: &Pixel = &self.grid.grid[col][row_upper];
                let upper_color = Color::Rgb(upper.color.red, upper.color.green, upper.color.blue);
                let lower_color = if row_lower < self.grid.height as usize {
                    // index guard for odd heights
                    let lower: &Pixel = &self.grid.grid[col][row_lower];
                    Color::Rgb(lower.color.red, lower.color.green, lower.color.blue)
                } else {
                    Color::Reset // will render a transparent row on the bottom of table
                    // when table_height is odd
                };

                let cell_opt = buf.cell_mut(Position::new(row as u16, col as u16));

                match cell_opt {
                    Some(mut cell) => {
                        cell = cell.set_char('▀');
                        cell.fg = upper_color;
                        cell.bg = lower_color;
                    }
                    None => {}
                }
            }
        }
    }
}
