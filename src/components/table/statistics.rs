use ratatui::{prelude::*, widgets::*};
use crate::tui::{stats, models};


pub fn render(f: &mut Frame, app: &mut models::App, area: Rect) {
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
