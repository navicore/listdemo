use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    prelude::*,
    widgets::{Block, Borders, Cell, Row, StatefulWidget, Table, TableState, Widget},
};
use std::{error::Error, io};
use tui_widget_list::{List, ListState, Listable};

#[derive(Debug, Clone)]
struct UiSection<'a> {
    items: Vec<Row<'a>>,
    content_table: Table<'a>,
    content_table_state: TableState,
    height: u16,
}

fn create_table<'a>(num_rows: usize, title: String) -> (Table<'static>, Vec<Row<'a>>) {
    // Generate the rows based on the input parameter
    let mut rows = Vec::new();
    for i in 0..num_rows {
        rows.push(Row::new(vec![
            format!("replicaname-{}", i + 1),
            "10/10".to_string(),
            "20/20".to_string(),
        ]));
    }

    // Define the column widths
    let widths = [
        Constraint::Percentage(50),
        Constraint::Max(30),
        Constraint::Min(10),
    ];

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    //let normal_style = Style::default().bg(Color::Blue);
    let normal_style = Style::default();
    let header_cells = ["Name", "Pods", "Containers"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells).style(normal_style).height(1);

    // Create and return the table
    (
        Table::new(rows.clone(), widths)
            .header(header)
            .block(Block::default().borders(Borders::ALL).title(title))
            .highlight_style(selected_style)
            .highlight_symbol(">>"),
        rows,
    )
}

// Usage
// let my_table = create_table(5); // Creates a table with 5 rows
impl UiSection<'_> {
    pub fn new(_text: &str, height: u16, title: String) -> Self {
        let content_table_state = TableState::default();
        let (content_table, items) = create_table(30, title);
        Self {
            items,
            content_table,
            content_table_state,
            height,
        }
    }

    pub fn next(&mut self) {
        let i = match self.content_table_state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.content_table_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.content_table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.content_table_state.select(Some(i));
    }

    pub fn style(mut self, style: Style) -> Self {
        self.content_table = self.content_table.set_style(style);
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

// impl<'a> StatefulWidget for UiSection<'a> {
//     type State = TableState;
//     fn render(
//         mut self,
//         area: Rect,
//         buf: &mut Buffer,
//         state: &mut <Table<'_> as StatefulWidget>::State,
//     ) {
//         // self.content_table
//         //     .render(self.content_table, area, buf, &mut self.content_table_state);
//         StatefulWidget::render(self.content_table, area, buf, &mut self.content_table_state);
//     }
// }

impl<'a> Widget for UiSection<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        StatefulWidget::render(self.content_table, area, buf, &mut self.content_table_state);
        //Widget::render(self.content_table, area, buf);
        //self.content_table.render(area, buf);
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

impl App<'static> {
    pub fn new() -> App<'static> {
        let items: Vec<UiSection> = vec![
            UiSection::new("Height: 12", 12, "ReplicaSets".to_string()),
            UiSection::new("Height: 8", 8, "Services".to_string()),
        ];
        let list = List::new(items.clone())
            .style(Style::default().bg(Color::Black))
            .block(Block::default().borders(Borders::ALL).title("Navipod"))
            .truncate(true);
        let state = ListState::default();
        App { items, list, state }
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),

                    KeyCode::BackTab => app.state.previous(),
                    KeyCode::Tab => app.state.next(),

                    KeyCode::Down | KeyCode::Char('j') => match app.state.selected() {
                        Some(idx) => {
                            let section = &mut app.items[idx];
                            section.next();
                        }
                        _ => {}
                    },
                    KeyCode::Up | KeyCode::Char('k') => match app.state.selected() {
                        Some(idx) => {
                            let section = &mut app.items[idx];
                            section.previous();
                        }
                        _ => {}
                    },

                    _ => {}
                }
            }
        }
    }
}

pub struct App<'a> {
    items: Vec<UiSection<'a>>,
    list: List<'a, UiSection<'a>>,
    state: ListState,
}

impl Default for App<'static> {
    fn default() -> Self {
        Self::new()
    }
}

fn render_selected<'a>(f: &mut Frame, app: &mut App<'a>) {
    let rects = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Max(30),
            Constraint::Min(10),
        ]);
    let area = rects.split(f.size());

    match app.state.selected() {
        Some(idx) => {
            let section = &mut app.items[idx];
            f.render_stateful_widget(
                section.content_table.clone(),
                area[0],
                &mut section.content_table_state,
            );
        }
        _ => {}
    };
}

pub fn ui<'a>(f: &mut Frame, app: &mut App<'a>) {
    render_selected(f, app);
    f.render_stateful_widget(app.list.clone(), f.size(), &mut app.state);
}
