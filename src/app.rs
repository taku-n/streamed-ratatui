use color_eyre::Result;
use futures::FutureExt; // fuse()
use futures::StreamExt; // next()
use tokio::sync::mpsc;

use crossterm::event::Event;
use crossterm::event::EventStream;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::widgets::Block;
use ratatui::widgets::Paragraph;
use ratatui::DefaultTerminal;
use ratatui::Frame;

#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    running: bool,

    string: String,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub async fn run(
        mut self,
        mut terminal: DefaultTerminal,
        rx: &mut mpsc::Receiver<i32>,
    ) -> Result<()> {
        let mut reader = EventStream::new();
        self.running = true;
        while self.running {
            // Rendering phase
            terminal.draw(|frame| self.draw(frame))?;

            // Event handling phase
            let event = reader.next().fuse(); // Returns kinda like Future immediately
            tokio::select! {  // https://docs.rs/tokio/latest/tokio/macro.select.html#cancellation-safety
                option_event = event => self.handle_crossterm_event(option_event).await?,
                option_time = rx.recv() => self.handle_time_event(option_time).await?,
            };
        }
        Ok(())
    }

    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/master/examples>
    fn draw(&mut self, f: &mut Frame) {
        f.render_widget(
            Paragraph::new(self.string.clone()).block(Block::new()),
            f.area(),
        );
    }

    /// Reads the crossterm events and updates the state of [`App`].
    async fn handle_crossterm_event(
        &mut self,
        option_event: Option<Result<Event, std::io::Error>>,
    ) -> Result<()> {
        if let Some(event) = option_event {
            match event? {
                // it's important to check KeyEventKind::Press to avoid handling key release events
                Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key).await,

                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    async fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            //(_, KeyCode::Esc | KeyCode::Char('q'))
            (_, KeyCode::Esc)
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit().await,
            // Add other key handlers here.
            (_, KeyCode::Char('a')) => self.string = "a".into(),
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    async fn quit(&mut self) {
        self.running = false;
    }

    async fn handle_time_event(&mut self, option_x: Option<i32>) -> Result<()> {
        if let Some(x) = option_x {
            self.string = x.to_string();
        }
        Ok(())
    }
}
