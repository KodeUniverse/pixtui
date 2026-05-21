use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, HorizontalAlignment, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, BorderType, Cell, Row, StatefulWidget, Table, TableState, Widget};

use crate::pixels::PixelGrid;

pub struct Editor {
    pub pixel_grid: PixelGrid,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            pixel_grid: PixelGrid::new(36, 36),
        }
    }
}

impl StatefulWidget for &Editor {
    type State = TableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut TableState)
    where
        Self: Sized,
    {
        let block = Block::bordered()
            .title_top(" Pixel Editor ")
            .title_alignment(HorizontalAlignment::Center)
            .border_type(BorderType::Thick);
        (&block).render(area, buf);

        let inner = block.inner(area);
        let grid_size = self.pixel_grid.grid.len();

        let rows: Vec<Row> = (0..grid_size)
            .map(|y| {
                let cells: Vec<Cell> = (0..grid_size)
                    .map(|x| {
                        let pixel = &self.pixel_grid.grid[x][y];
                        let style = Style::default().bg(Color::Rgb(
                            pixel.color.red,
                            pixel.color.green,
                            pixel.color.blue,
                        ));
                        Cell::from("  ").style(style)
                    })
                    .collect();
                Row::new(cells)
            })
            .collect();

        let widths = vec![Constraint::Ratio(1, grid_size as u32); grid_size];

        let table = Table::new(rows, widths)
            .column_spacing(0)
            .cell_highlight_style(
                Style::default()
                    .fg(Color::White)
                    .bg(Color::Black)
                    .add_modifier(Modifier::REVERSED),
            );

        StatefulWidget::render(table, inner, buf, state);
    }
}
