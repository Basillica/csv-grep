use std::{error::Error, io};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use crate::terminal::{table::models, stats};

const INFO_TEXT: &str =
    "(Esc) quit | (↑) move up | (↓) move down | (→) next color | (←) previous color | ↲ for Menu";

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
            render_table(f, app, inner_layout[2]);
            render_scrollbar(f, app, inner_layout[2]);
        },
        "Visualization" => {
            let horizontal = Layout::horizontal([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)]);
            let [line_chart, scatter]  = horizontal.areas(inner_layout[2]);
            render_line_chart(f, app, line_chart);
            render_scatter(f, app, scatter);
        },
        "Statistics" => {
            render_statistics_table(f, app, inner_layout[2]);
            render_statistics_scrollbar(f, app, inner_layout[2]);
        },
        "Extras" => {},
        _ => {}
    }

    // left side
    render_menu_items(f, app, inner_layout[0]);
    render_menu_scrollbar(f, app, inner_layout[0]);
    // footer
    render_footer(f, app, outer_layout[2]);
}


fn render_statistics_table(f: &mut Frame, app: &mut models::App, area: Rect) {
    let header_style = Style::default()
        .fg(app.colors.header_fg)
        .bg(app.colors.header_bg);
    let selected_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(app.colors.selected_style_fg);

    let header = app.stats_header
        .iter()
        .cloned()
        .map(Cell::from)
        .collect::<Row>()
        .style(header_style)
        .height(2);

    let mut cols:  Vec<Vec<String>> = vec![
        vec!["0".to_string(), "mean".to_string()], vec!["1".to_string(), "median".to_string()],
        vec!["2".to_string(), "range".to_string()], vec!["3".to_string(), "varaince".to_string()],
        vec!["4".to_string(), "standard deviation".to_string()], vec!["5".to_string(), "percentile 25".to_string()],
        vec!["6".to_string(), "percentile 50".to_string()], vec!["7".to_string(), "percentile 75".to_string()],
        vec!["8".to_string(), "skewness".to_string()], vec!["9".to_string(), "kurtosis".to_string()]
    ];


    for el in &app.raw_data {
        let statistics = stats::Data{
            data: el.data.clone(),
        };
        let (v, std) = statistics.variance_n_std();
        let (p25, p50, p75) = statistics.percentiles();
        let stats: Vec<f64> = vec![
            statistics.mean(),  p50,
            statistics.range().unwrap(), v,
            std, p25,  p50,  p75, statistics.skewness(),
            statistics.kurtosis(),
        ];

        for (j, stat) in stats.iter().enumerate() {
            let a = stat.to_string();
            cols[j].push(a)
        }
    }

    let rows = cols.iter().enumerate().map(|(i, data)| {
        let color = match i % 2 {
            0 => app.colors.normal_row_color,
            _ => app.colors.alt_row_color,
        };

        let item: Vec<&str> = data.iter().map(|f: &String| f.as_str()).collect();
        item.iter()
            .cloned()
            .map(|content| Cell::from(Text::from(format!("\n{}\n", content))))
            .collect::<Row>()
            .style(Style::new().fg(app.colors.row_fg).bg(color))
            .height(2)
    });


    let bar = " ⮞ ";
    let mut width: Vec<Constraint> = [].to_vec();
    let space = 100/app.table_header.len();
    for (_, _) in app.table_header.iter().enumerate() {
        width.push(Constraint::Percentage(space as u16))
    }
    
    
    let t = Table::new(rows, width)
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


fn render_statistics_scrollbar(f: &mut Frame, app: &mut models::App, area: Rect) {
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
        // println!("{:?}", item);
        item.iter()
            .cloned()
            .map(|content| Cell::from(Text::from(format!("\n{}\n", content))))
            .collect::<Row>()
            .style(Style::new().fg(app.colors.row_fg).bg(color))
            .height(2)
    });
    let bar = " ⮞ ";
    let mut width: Vec<Constraint> = [].to_vec();
    let space = 100/app.table_header.len();
    for (_, _) in app.table_header.iter().enumerate() {
        width.push(Constraint::Percentage(space as u16))
    }


    let t = Table::new(rows, width)
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
        item.iter()
            .cloned()
            .map(|content| Cell::from(Text::from(format!("\n{}\n", content))))
            .collect::<Row>()
            .style(Style::new().fg(app.colors.row_fg).bg(color).bold())
            .height(4)
    });
    let bar = " █ ";
    let t = Table::new(
        rows,
        [
            Constraint::Length(app.longest_menu_item_len),
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


pub fn render_line_chart(f: &mut Frame, app: &mut models::App, area: Rect) {
    let mut a: Vec<Dataset<'_>> = [].to_vec();
    let styles = [
        Color::Black,
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::Gray,
        Color::DarkGray,
        Color::LightRed,
        Color::LightGreen,
        Color::LightYellow,
        Color::LightBlue,
        Color::LightMagenta,
    ];
    let mut iter = 0;
    for data in &app.plot_data {
        let h = &app.grouped_headers[iter];
        a.push(
            Dataset::default()
                .name(format!("plot of {} against {}", h.0, h.1))
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(styles[iter]))
                .graph_type(GraphType::Line)
                .data(&data)
            );
            iter += 1;
    }

    let ((min_x, min_y), (max_x, max_y)) = app.plot_data.iter().flat_map(|v| v.iter()).fold(
        ((f64::INFINITY, f64::INFINITY), (f64::NEG_INFINITY, f64::NEG_INFINITY)),
        |((min_x, min_y), (max_x, max_y)), &(x, y)| {
            ((min_x.min(x), min_y.min(y)), (max_x.max(x), max_y.max(y)))
        },
    );

    let chart = Chart::new(a)
        .block(
            Block::default()
                .title(
                    block::Title::default()
                        .content("Line chart".cyan().bold())
                        .alignment(Alignment::Center),
                )
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("X Axis")
                .style(Style::default().gray())
                .bounds([min_x-10.0, max_x+10.0])
                .labels(vec![format!("{min_x}").into(), format!("{max_x}").into()]),
        )
        .y_axis(
            Axis::default()
                .title("Y Axis")
                .style(Style::default().gray())
                .bounds([min_y-10.0, max_y+10.0])
                .labels(vec![format!("{min_y}").into(), format!("{max_y}").into()]),
        )
        .legend_position(Some(LegendPosition::TopLeft))
        .hidden_legend_constraints((Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)));

    f.render_widget(chart, area)
}

pub fn render_scatter(f: &mut Frame, app: &mut models::App, area: Rect) {
    let styles = [
        Style::new().yellow(),
        Style::new().green(),
        Style::new().blue(),
        Style::new().gray(),
        Style::new().cyan(),
        Style::new().yellow(),
        Style::new().green(),
        Style::new().yellow(),
        Style::new().green(),
    ];

    let mut a: Vec<Dataset<'_>> = [].to_vec();
    let mut iter = 0;
    for data in &app.plot_data {
        let h = &app.grouped_headers[iter];
        a.push(
            Dataset::default()
            .name(format!("plot of {} against {}", h.0, h.1))
                .marker(Marker::Dot)
                .graph_type(GraphType::Scatter)
                .style(styles[iter])
                .data(&data)
        );
        iter += 1;
    }

    let ((min_x, min_y), (max_x, max_y)) = app.plot_data.iter().flat_map(|v| v.iter()).fold(
        ((f64::INFINITY, f64::INFINITY), (f64::NEG_INFINITY, f64::NEG_INFINITY)),
        |((min_x, min_y), (max_x, max_y)), &(x, y)| {
            ((min_x.min(x), min_y.min(y)), (max_x.max(x), max_y.max(y)))
        },
    );

    let chart = Chart::new(a)
        .block(
            Block::new().borders(Borders::all()).title(
                block::Title::default()
                    .content("Scatter chart".cyan().bold())
                    .alignment(Alignment::Center),
            ),
        )
        .x_axis(
            Axis::default()
                .title("Year")
                .bounds([min_x-10.0, max_x+10.0])
                .style(Style::default().fg(Color::Gray))
                .labels(vec![format!("{min_x}").into(), format!("{max_x}").into()]),
        )
        .y_axis(
            Axis::default()
                .title("Cost")
                .bounds([min_y-10.0, max_y+10.0])
                .style(Style::default().fg(Color::Gray))
                .labels(vec![format!("{min_y}").into(), format!("{max_y}").into()]),
        )
        .hidden_legend_constraints((Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)));

    f.render_widget(chart, area);
}
