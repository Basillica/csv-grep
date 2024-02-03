use std::{error::Error, io};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use crate::terminal::table::models;

const INFO_TEXT: &str =
    "(Esc) quit | (↑) move up | (↓) move down | (→) next color | (←) previous color";

use color_eyre:: Result;



pub fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = models::App::new();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

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
            render_table(f, app, inner_layout[2]);
            render_scrollbar(f, app, inner_layout[2]);
        },
        "Visualization" => {},
        "Statistics" => {},
        "Extras" => {},
        _ => {}
    }
    
    // left side
    render_menu_items(f, app, inner_layout[0]);
    render_menu_scrollbar(f, app, inner_layout[0]);
    // footer
    render_footer(f, app, outer_layout[2]);
}


fn render_table(f: &mut Frame, app: &mut models::App, area: Rect) {
    let header_style = Style::default()
        .fg(app.colors.header_fg)
        .bg(app.colors.header_bg);
    let selected_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(app.colors.selected_style_fg);

    let header = app.table_header
        .iter()
        .cloned()
        .map(Cell::from)
        .collect::<Row>()
        .style(header_style)
        .height(2);

    let rows = app.items.iter().enumerate().map(|(i, data)| {
        let color = match i % 2 {
            0 => app.colors.normal_row_color,
            _ => app.colors.alt_row_color,
        };

        let item: Vec<&str> = data.iter().collect();
        item.iter()
            .cloned()
            .map(|content| Cell::from(Text::from(format!("\n{}\n", content))))
            .collect::<Row>()
            .style(Style::new().fg(app.colors.row_fg).bg(color))
            .height(2)
    });
    let bar = "⮞ ";
    let mut width: Vec<Constraint> = [].to_vec();
    let space = 100/app.table_header.len();
    for (_, _) in app.table_header.iter().enumerate() {
        width.push(Constraint::Percentage(space as u16))
    }


    let t = Table::new(
        rows,
        width,
    )
    .header(header)
    .highlight_style(selected_style)
    .highlight_symbol(Text::from(vec![
        "".into(),
        bar.into(),
        bar.into(),
        "".into(),
    ]))
    .bg(app.colors.buffer_bg)
    .highlight_spacing(HighlightSpacing::Always);
    f.render_stateful_widget(t, area, &mut app.app_state);
}


fn render_scrollbar(f: &mut Frame, app: &mut models::App, area: Rect) {
    f.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None),
        area.inner(&Margin {
            vertical: 1,
            horizontal: 1,
        }),
        &mut app.scroll_state,
    );
}


fn render_menu_items(f: &mut Frame, app: &mut models::App, area: Rect) {
    let header_style = Style::default()
        .fg(app.colors.header_fg)
        .bg(app.colors.header_bg);
    let selected_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(app.colors.selected_style_fg);

    let header = ["Menu"]
        .iter()
        .cloned()
        .map(Cell::from)
        .collect::<Row>()
        .style(header_style)
        .height(2);
    let rows = app.menu_items.iter().enumerate().map(|(i, data)| {
        let color = match i % 2 {
            0 => app.colors.normal_row_color,
            _ => app.colors.alt_row_color,
        };
        let item = [data];
        // let item = data.ref_array();
        item.iter()
            .cloned()
            .map(|content| Cell::from(Text::from(format!("\n{}\n", content))))
            .collect::<Row>()
            .style(Style::new().fg(app.colors.row_fg).bg(color))
            .height(4)
    });
    let bar = " █ ";
    let t = Table::new(
        rows,
        [
            // + 1 is for padding.
            Constraint::Length(app.longest_menu_item_len),
            // Constraint::Min(app.longest_item_lens.1 + 1),
            // Constraint::Min(app.longest_item_lens.2),
        ],
    )
    .header(header)
    .highlight_style(selected_style)
    .highlight_symbol(Text::from(vec![
        "".into(),
        bar.into(),
        bar.into(),
        "".into(),
    ]))
    .bg(app.colors.buffer_bg)
    .highlight_spacing(HighlightSpacing::Always);
    f.render_stateful_widget(t, area, &mut app.menu_state);
}


fn render_menu_scrollbar(f: &mut Frame, app: &mut models::App, area: Rect) {
    f.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None),
        area.inner(&Margin {
            vertical: 1,
            horizontal: 1,
        }),
        &mut app.menu_scroll_state,
    );
}


fn render_footer(f: &mut Frame, app: &mut models::App, area: Rect) {
    let info_footer = Paragraph::new(Line::from(INFO_TEXT))
        .style(Style::new().fg(app.colors.row_fg).bg(app.colors.buffer_bg))
        .centered()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::new().fg(app.colors.footer_border_color))
                .border_type(BorderType::Double),
        );
    f.render_widget(info_footer, area);
}
