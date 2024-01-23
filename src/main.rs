use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    prelude::*,
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};
use std::{error::Error, io};
use tui_widget_list::{List, ListState, Listable};

pub trait UiSectionTrait<'a>: StatefulWidget + Widget + Listable {
    fn render(&self, area: Rect, buf: &mut Buffer);
    //fn height(&self) -> u16;
}

#[derive(Debug)]
struct Replicasets<'a> {
    paragraph: Paragraph<'a>,
    height: u16,
    state: ListState,
}

impl<'a> Replicasets<'a> {
    pub fn new(text: &str, height: u16, title: &'a str) -> Self {
        let paragraph = Paragraph::new(vec![Line::from(Span::styled(
            text.to_string(),
            Style::default().fg(Color::Cyan),
        ))])
        .style(Style::default().bg(Color::Black))
        .block(Block::default().borders(Borders::ALL).title(title));
        let state = ListState::default();
        Self {
            paragraph,
            height,
            state,
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.paragraph = self.paragraph.set_style(style);
        self
    }
}

impl<'a> StatefulWidget for Replicasets<'a> {
    type State = ListState;
    fn render(self, _area: Rect, _buf: &mut Buffer, _state: &mut Self::State) {}
}

impl<'a> UiSectionTrait<'a> for Replicasets<'a> {
    fn render(&self, _area: Rect, _buf: &mut Buffer) {
        // Implementation for rendering Replicasets
    }

    // fn height(&self) -> u16 {
    //     // Return height for Replicasets
    //     self.height
    // }
}

impl<'a> Listable for Replicasets<'a> {
    fn height(&self) -> usize {
        self.height as usize
    }
    fn highlight(self) -> Self {
        let style = Style::default()
            .fg(Color::Yellow) // Changing the foreground color to yellow
            .bg(Color::Black); // Keeping the background color same
        self.style(style)
    }
}

impl<'a> Widget for Replicasets<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.paragraph.render(area, buf);
    }
}

#[derive(Debug)]
struct Services<'a> {
    paragraph: Paragraph<'a>,
    height: u16,
    state: ListState,
}

impl<'a> Widget for Services<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.paragraph.render(area, buf);
    }
}

impl<'a> StatefulWidget for Services<'a> {
    type State = ListState;
    fn render(self, _area: Rect, _buf: &mut Buffer, _state: &mut Self::State) {}
}

impl<'a> UiSectionTrait<'a> for Services<'a> {
    fn render(&self, _area: Rect, _buf: &mut Buffer) {
        // Implementation for rendering Services
    }

    // fn height(&self) -> u16 {
    //     self.height
    // }
}

impl<'a> Listable for Services<'a> {
    fn height(&self) -> usize {
        self.height as usize
    }
    fn highlight(self) -> Self {
        let style = Style::default()
            .fg(Color::Yellow) // Changing the foreground color to yellow
            .bg(Color::Black); // Keeping the background color same
        self.style(style)
    }
}

impl<'a> Services<'a> {
    pub fn new(text: &str, height: u16, title: &'a str) -> Self {
        let paragraph = Paragraph::new(vec![Line::from(Span::styled(
            text.to_string(),
            Style::default().fg(Color::Cyan),
        ))])
        .style(Style::default().bg(Color::Black))
        .block(Block::default().borders(Borders::ALL).title(title));
        let state = ListState::default();
        Self {
            paragraph,
            height,
            state,
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.paragraph = self.paragraph.set_style(style);
        self
    }
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut terminal = init_terminal()?;

    let app = App::new();
    run_app(&mut terminal, app).unwrap();

    reset_terminal()?;
    terminal.show_cursor()?;

    Ok(())
}

/// Initializes the terminal.
fn init_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    crossterm::execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(io::stdout());

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    panic_hook();

    Ok(terminal)
}

/// Resets the terminal.
fn reset_terminal() -> Result<()> {
    disable_raw_mode()?;
    crossterm::execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;

    Ok(())
}

/// Shutdown gracefully
fn panic_hook() {
    let original_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic| {
        reset_terminal().unwrap();
        original_hook(panic);
    }));
}

impl<'a> Widget for Box<dyn UiSectionTrait<'a, State = ListState>> {
    fn render(self, _area: Rect, _buf: &mut Buffer) {
        // Implementation for rendering Replicasets
    }
}

impl<'a> Listable for Box<dyn UiSectionTrait<'a, State = ListState>> {
    // Implement the methods required by the Listable trait
    // You'll need to delegate these method calls to the inner trait object
    // For example:

    fn height(&self) -> usize {
        self.as_ref().height()
    }

    fn highlight(self) -> Self {
        // Implement highlighting for the boxed trait object
        // This might be tricky and require additional thought
        // based on how your trait objects should handle highlighting
        self
    }
}

impl App<'static> {
    pub fn new() -> App<'static> {
        let items: Vec<Box<dyn UiSectionTrait<State = ListState>>> = vec![
            Box::new(Replicasets::new("Height: 12", 12, "ReplicaSets")),
            Box::new(Services::new("Height: 8", 8, "Services")),
        ];
        let list = List::new(items)
            .style(Style::default().bg(Color::Black))
            .block(Block::default().borders(Borders::ALL).title("Navipod"))
            .truncate(true);
        let state = ListState::default();
        App { list, state }
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Up => app.state.previous(),
                    KeyCode::Down => app.state.next(),
                    _ => {}
                }
            }
        }
    }
}

pub struct App<'a> {
    pub list: List<'a, Box<dyn UiSectionTrait<'a, State = ListState>>>,
    pub state: ListState,
}

impl Default for App<'static> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> StatefulWidget for Box<dyn UiSectionTrait<'a, State = ListState>> {
    type State = ListState;
    fn render(self, _area: Rect, _buf: &mut Buffer, _state: &mut Self::State) {}
}

pub fn ui<'a>(f: &mut Frame, app: &mut App<'a>) {
    //f.render_stateful_widget(&mut app.list, f.size(), &mut app.state);
    //f.render_widget(&mut app.list, &mut app.state);
    f.render_widget(&mut app.list, area);
}
