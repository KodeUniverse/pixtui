use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, BorderType, List, ListItem, Paragraph, Widget},
};
#[derive(Debug, Default)]
enum Route {
    #[default]
    Home,
    Editor,
}

#[derive(Debug)]
pub struct App {
    route: Route,
    exit: bool,
}
impl Default for App {
    fn default() -> Self {
        Self {
            route: Route::Home,
            exit: false,
        }
    }
}
impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_home(&self, area: Rect, buf: &mut Buffer) {
        let items = [
            ListItem::new("Create new project"),
            ListItem::new("Open existing project"),
            ListItem::new("Options"),
        ];
        let vert = Layout::vertical([Constraint::Fill(1), Constraint::Percentage(40)]).split(area);
        let block_slot = Rect::new(area.width / 2 - 25, vert[1].y, 50, 10);
        let block = Block::bordered().border_set(border::THICK);
        Line::from("PIXTERM")
            .centered()
            .render(vert[0].centered_vertically(Constraint::Percentage(30)), buf);
        List::new(items)
            .highlight_symbol(">")
            .block(block)
            .render(block_slot, buf);
    }
    fn render_editor(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().border_set(border::THICK);
        block.render(area, buf);
    }
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        }
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Up => {}
            _ => {}
        }
    }
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.route {
            Route::Home => {
                self.render_home(area, buf);
            }
            Route::Editor => {
                self.render_editor(area, buf);
            }
        }
    }
}

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))
}
