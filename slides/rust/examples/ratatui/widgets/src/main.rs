use std::io;

use crossterm::event::KeyModifiers;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, KeyCode, KeyEventKind},
    layout::Rect,
    style::Stylize,
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
    text::{Line, Text},
    symbols::border,
};

#[derive(Debug, Default)]
struct App {
    counter: u8,
    exit: bool,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

            if let event::Event::Key(key) = event::read()? {
                self.handle_key_event(key);
            }
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
                KeyCode::Right => self.counter = self.counter.saturating_add(1),
                KeyCode::Left => self.counter = self.counter.saturating_sub(1),
                _ => {}
            }
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.counter {
            1 => Paragraph::new(format!("{}", self.counter))
                .white()
                .on_blue()
                .render(area, buf),
            2 => Paragraph::new(format!("{}", self.counter))
                .black()
                .on_green()
                .render(area, buf),
            3 => {
                let text = Text::from(vec![Line::from(vec![
                    "Centered Value: ".to_string().black().on_white(),
                    self.counter.to_string().red(),
                ])]);
        
                Paragraph::new(text)
                    .centered()
                    .on_light_blue()
                    .render(area, buf);
        
            },

            0 => {
                let title = Line::from(" Counter App Tutorial ".bold());
                let instructions = Line::from(vec![
                    " Decrement ".into(),
                    "<Left>".blue().bold(),
                    " Increment ".into(),
                    "<Right>".blue().bold(),
                    " Quit ".into(),
                    "<Q> ".blue().bold(),
                ]);
                let block = Block::bordered()
                    .title(title.centered())
                    .title_bottom(instructions.centered())
                    .border_set(border::THICK);
        
                let counter_text = Text::from(vec![Line::from(vec![
                    "Value: ".into(),
                    self.counter.to_string().yellow(),
                ])]);
        
                Paragraph::new(counter_text)
                    .centered()
                    .block(block)
                    .render(area, buf);
                
            },

            _ => Paragraph::new(format!("Not implemented: {} Press the left and right arrows to cycle throught the examples.", self.counter))
                .white()
                .on_blue()
                .render(area, buf),
        }
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

