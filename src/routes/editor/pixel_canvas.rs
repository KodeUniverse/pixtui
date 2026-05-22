use crate::pixels::{Pixel, PixelGrid};
use ratatui::buffer::{Buffer, Cell};
use ratatui::layout::{Position, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::StatefulWidget;

pub struct PixelCanvasState {
    pub x: u16,
    pub y: u16,
}
pub struct PixelCanvas {
    pub marker: PixelCanvasState,
    pub grid: PixelGrid,
}

impl PixelCanvas {
    fn move_marker_up(&mut self, by: u16) {
        self.marker.y -= by;
    }
    fn move_marker_down(&mut self, by: u16) {
        self.marker.y += by;
    }
    fn move_marker_right(&mut self, by: u16) {
        self.marker.x += by;
    }
    fn move_marker_left(&mut self, by: u16) {
        self.marker.x -= by;
    }
}

impl StatefulWidget for PixelCanvas {
    type State = PixelCanvasState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut PixelCanvasState) {
        let table_height = ((self.grid.height + 1) / 2) as usize;
        let table_width = self.grid.width as usize;

        for i in 0..table_height {
            for j in 0..table_width {
                let cell = buf.cell_mut(Position::new(i as u16, j as u16)).or_else(|| None);
            }
        }

}
