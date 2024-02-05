use ratatui::{prelude::*, widgets::*};


pub fn render(f: &mut Frame, scroll_state: &mut ScrollbarState, area: Rect) {
    f.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None),
        area.inner(&Margin {
            vertical: 1,
            horizontal: 1,
        }),
        scroll_state,
    );
}