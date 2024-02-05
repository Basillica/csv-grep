use std::{error::Error, io};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use crate::tui::models;
use crate::components::{charts, menu, table, utils};

use color_eyre::Result;



pub fn main(file_path: String) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = models::App::new(file_path);
    let _ = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}


fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: models::App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                use KeyCode::*;
                match key.code {
                    Char('q') | Esc => return Ok(()),
                    Char('j') | Down => app.next(),
                    Char('k') | Up => app.previous(),
                    Char('l') | Right => app.next_color(),
                    Char('h') | Left => app.previous_color(),
                    Char('t') | Enter => app.next_menu(),
                    Char('b') | BackTab => app.previous_menu(),
                    _ => {}
                }
            }
        }
    }
}


pub fn ui(f: &mut Frame, app: &mut models::App) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(f.size());

    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(20),
            Constraint::Percentage(1),
            Constraint::Percentage(79),
        ])
        .split(outer_layout[1]);

    f.render_widget(
        Paragraph::new("CSV Parser").style(Style::new().fg(app.colors.row_fg).bg(app.colors.buffer_bg))
            .centered()
            .block(Block::new().borders(Borders::ALL)
            .border_style(Style::new().fg(app.colors.footer_border_color))
            .border_type(BorderType::Double)),
        outer_layout[0]);

    f.render_widget(
        Paragraph::new("")
            .block(Block::new().borders(Borders::ALL)),
            inner_layout[1]);

    app.set_colors();

    // right side
    match app.tab {
        "Data Explorer" => {
            table::table::render(f, app, inner_layout[2]);
            utils::scroll_bar::render(f, &mut app.scroll_state, inner_layout[2]);
        },
        "Visualization" => {
            let horizontal = Layout::horizontal([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)]);
            let [line_chart, scatter]  = horizontal.areas(inner_layout[2]);
            charts::line_chart::render(f, app, line_chart);
            charts::scatter_plot::render(f, app, scatter);
        },
        "Statistics" => {
            table::statistics::render(f, app, inner_layout[2]);
            utils::scroll_bar::render(f, &mut app.scroll_state, inner_layout[2]);
        },
        "Extras" => {},
        _ => {}
    }

    // left side
    menu::items::render(f, app, inner_layout[0]);
    utils::scroll_bar::render(f, &mut app.menu_scroll_state, inner_layout[0]);
    // footer
    utils::footer::render(f, app, outer_layout[2]);
}