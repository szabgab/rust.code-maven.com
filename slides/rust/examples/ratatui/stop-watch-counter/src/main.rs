use std::io;

use crossterm::event::KeyModifiers;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, KeyCode, KeyEventKind},
    layout::Rect,
    style::Stylize,
    widgets::{Paragraph, Widget},
    DefaultTerminal, Frame,
};

#[derive(Debug, Default)]
struct App {
    counter: u32,
    text: String,
    exit: bool,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

            // Blocking read
            if let event::Event::Key(key) = event::read()? {
                self.handle_key_event(key);
            }

            // Poll before blocking read
            // if event::poll(std::time::Duration::from_millis(100))? {
            //     if let event::Event::Key(key) = event::read()? {
            //         self.handle_key_event(key);
            //     }
            // }

            // Short poll
            // if event::poll(std::time::Duration::from_millis(1))? {
            //     if let event::Event::Key(key) = event::read()? {
            //         self.handle_key_event(key);
            //     }
            // }
            // std::thread::sleep(std::time::Duration::from_millis(100));

            self.counter += 1;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame<'_>) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_event(&mut self, key: event::KeyEvent) {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Esc => self.exit = true,
                KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => self.exit = true,
                _ => self.text = format!("{} - {}", key.modifiers, key.code),
            }
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(format!("{}: {}", self.counter, self.text))
            .white()
            .on_blue()
            .render(area, buf);
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
