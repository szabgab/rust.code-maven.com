use std::io;

use crossterm::event::KeyModifiers;
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,
    DefaultTerminal,
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = run(terminal);
    ratatui::restore();
    app_result
}

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    let mut text = String::new();
    loop {
        terminal.draw(|frame| {
            let greeting = Paragraph::new(text.clone()).white().on_blue();
            frame.render_widget(greeting, frame.area());
        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                // if key.code == KeyCode::Char('q') {
                //     return Ok(());
                // }

                //text.push_str("a");

                if key.code == KeyCode::Esc {
                    return Ok(());
                }
                if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
                    return Ok(());
                }

                // // println!("{}", key.modifiers);
                // if key.modifiers == KeyModifiers::SHIFT {
                //     text.push_str("Shift+");
                //     text.push_str(&key.code.to_string());
                // } else {
                //     text.push_str(&key.code.to_string());
                // }

                //text.push_str(&key.code.to_string());
                text = format!("{} - {}", key.modifiers, key.code);
            }
        }
    }
}
