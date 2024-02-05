use ratatui::{prelude::*, widgets::*};
use crate::tui::models;

pub fn render(f: &mut Frame, app: &mut models::App, area: Rect) {
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
