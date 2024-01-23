use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    prelude::*,
    widgets::{Block, Borders, Paragraph, Widget},
};
use std::{error::Error, io};
use tui_widget_list::{List, ListState, Listable};

#[derive(Debug, Clone)]
struct UiSection<'a> {
    paragraph: Paragraph<'a>,
    height: u16,
}

impl UiSection<'_> {
    pub fn new(text: &str, height: u16, title: String) -> Self {
        let paragraph = Paragraph::new(vec![Line::from(Span::styled(
            text.to_string(),
            Style::default().fg(Color::Cyan),
        ))])
        .style(Style::default().bg(Color::Black))
        .block(Block::default().borders(Borders::ALL).title(title));
        Self { paragraph, height }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.paragraph = self.paragraph.set_style(style);
        self
    }
}

impl Listable for UiSection<'_> {
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

impl Widget for UiSection<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.paragraph.render(area, buf);
    }
}

// impl<'a> StatefulWidget for UiSection<'a> {
//     type State = ListState;
//     fn render(self, _area: Rect, _buf: &mut Buffer, _state: &mut Self::State) {}
// }
//
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

impl App<'static> {
    pub fn new() -> App<'static> {
        let items: Vec<UiSection> = vec![
            UiSection::new("Height: 12", 12, "ReplicaSets".to_string()),
            UiSection::new("Height: 8", 8, "Services".to_string()),
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
    list: List<'a, UiSection<'a>>,
    state: ListState,
}

impl Default for App<'static> {
    fn default() -> Self {
        Self::new()
    }
}

pub fn ui<'a>(f: &mut Frame, app: &mut App<'a>) {
    f.render_stateful_widget(app.list.clone(), f.size(), &mut app.state);
}
