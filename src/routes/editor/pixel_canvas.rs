use crate::pixels::PixelGrid;
use log::info;

use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::style::Color;
use ratatui::widgets::Widget;

pub struct Cursor {
    pub x: u16,
    pub y: u16,
}

pub struct PixelCanvas {
    pub grid: PixelGrid,
    pub canvas_h: u16,
    pub canvas_w: u16,
    pub cursor: Cursor,
}

impl PixelCanvas {
    pub fn new(grid_w: u16, grid_h: u16) -> Self {
        Self {
            grid: PixelGrid::new(grid_w, grid_h),
            canvas_w: grid_w,
            canvas_h: (grid_h + 1) / 2,
            cursor: Cursor { x: 0, y: 0 },
        }
    }
    pub fn move_select_up(&mut self, by: u16) {
        self.cursor.y = self.cursor.y.saturating_sub(by);
    }
    pub fn move_select_down(&mut self, by: u16) {
        self.cursor.y = self.cursor.y.saturating_add(by).min(self.canvas_h - 1);
    }
    pub fn move_select_right(&mut self, by: u16) {
        self.cursor.x = self.cursor.x.saturating_add(by).min(self.canvas_w - 1);
    }
    pub fn move_select_left(&mut self, by: u16) {
        self.cursor.x = self.cursor.x.saturating_sub(by);
    }
}

impl Default for PixelCanvas {
    fn default() -> Self {
        Self {
            canvas_w: 64,
            canvas_h: 32,
            cursor: Cursor { x: 0, y: 0 },
            grid: PixelGrid::new(64, 64),
        }
    }
}

impl Widget for &mut PixelCanvas {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let rows = (self.grid.height + 1) / 2; // to handle odd values also
        let cols = self.grid.width;

        let x_off = area.x + (area.width.saturating_sub(cols)) / 2;
        let y_off = area.y + (area.height.saturating_sub(rows)) / 2;

        for row in 0..rows {
            let row_upper = row as usize * 2;
            let row_lower = row_upper + 1;
            for col in 0..cols {
                let upper_color: Color = self.grid.get(col as u16, row_upper as u16).color.into();

                let lower_color = if row_lower < self.grid.height as usize {
                    self.grid.get(col as u16, row_lower as u16).color.into()
                } else {
                    Color::Reset
                };

                if let Some(cell) = buf.cell_mut(Position::new(x_off + col, y_off + row)) {
                    cell.set_char('▀');
                    cell.fg = upper_color;
                    cell.bg = lower_color;
                }
            }
        }
        // render selection marker
        let mut neighbor_render = |x: u16, y: u16| {
            let above = if y != 0 { (x, y - 1) } else { (x, 0) };
            let below = if y != self.canvas_h {
                (x, y + 1)
            } else {
                (x, self.canvas_h)
            };

            let left = if x != 0 { (x - 1, y) } else { (0, y) };
            let right = if x != self.canvas_w {
                (x + 1, y)
            } else {
                (self.canvas_w, y)
            };

            //above
            if let Some(cell) = buf.cell_mut(Position::new(x_off + above.0, y_off + above.1)) {
                cell.bg = Color::White;
            }
            //left
            if let Some(cell) = buf.cell_mut(Position::new(x_off + left.0, y_off + left.1)) {
                cell.set_char('▌').set_bg(Color::White);
            }
            //right
            if let Some(cell) = buf.cell_mut(Position::new(x_off + right.0, y_off + right.1)) {
                cell.set_char('▌').set_fg(Color::White);
            }
            //below
            if let Some(cell) = buf.cell_mut(Position::new(x_off + below.0, y_off + below.1)) {
                cell.fg = Color::White;
            }
        };
        let (select_x, select_y) = (self.cursor.x, self.cursor.y);

        info!(
            "terminal ({select_x}, {select_y}) = pixel rows {}–{}",
            select_y * 2,
            select_y * 2 + 1
        );

        neighbor_render(select_x, select_y);
    }
}
