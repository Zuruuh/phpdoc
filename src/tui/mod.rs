use crate::parser::XmlParser;
use ansi_to_tui::IntoText;
use bat::PrettyPrinter;
use color_eyre::Result;
use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use futures_util::{FutureExt, StreamExt};
use ratatui::{
    DefaultTerminal,
    crossterm::event,
    prelude::*,
    widgets::{Block, Padding, Paragraph, block::Position},
};
use screen::HomeScreen;
use std::io;
use text::ToLine;

mod screen;

#[derive(Default, Debug)]
pub struct TerminalState {
    event_stream: EventStream,
    running: bool,
    screen: Screen,
}

#[derive(Default, Debug)]
enum Screen {
    #[default]
    Home,
}

impl TerminalState {
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_crossterm_events().await?;
        }
        Ok(())
    }

    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/master/examples>
    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let block = Block::bordered().title(
            Line::from(format!("PHP DocBook {}", env!("CARGO_PKG_VERSION")))
                .bold()
                .blue()
                .centered(),
        );
        let container = block.inner(area);
        let buf = frame.buffer_mut();
        block.render(area, buf);

        match self.screen {
            Screen::Home => {
                HomeScreen.render(container, buf);
            }
        }
    }

    /// Reads the crossterm events and updates the state of [`App`].
    async fn handle_crossterm_events(&mut self) -> Result<()> {
        tokio::select! {
            event = self.event_stream.next().fuse() => {
                match event {
                    Some(Ok(evt)) => {
                        match evt {
                            Event::Key(key)
                                if key.kind == KeyEventKind::Press
                                    => self.on_key_event(key),
                            Event::Mouse(_) => {}
                            Event::Resize(_, _) => {}
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ = tokio::time::sleep(tokio::time::Duration::from_millis(100)) => {
                // Sleep for a short duration to avoid busy waiting.
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}
