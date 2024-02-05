use ratatui::{prelude::*, widgets::*};
use crate::tui::models;


pub fn render(f: &mut Frame, app: &mut models::App, area: Rect) {
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