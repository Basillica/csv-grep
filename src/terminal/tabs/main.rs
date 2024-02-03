use crate::terminal::{tabs::models::*, table};
use ratatui::{prelude::*, style::palette::tailwind, widgets::*};


impl SelectedTab {
    /// Get the previous tab, if there is no previous tab return the current tab.
    pub fn previous(&self) -> Self {
        let current_index: usize = *self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(*self)
    }

    /// Get the next tab, if there is no next tab return the current tab.
    pub fn next(&self) -> Self {
        let current_index = *self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(*self)
    }
}


impl Widget for SelectedTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // in a real app these might be separate widgets
        match self {
            SelectedTab::Tab1 => self.render_tab0(area, buf),
            SelectedTab::Tab2 => self.render_tab1(area, buf),
            SelectedTab::Tab3 => self.render_tab2(area, buf),
            SelectedTab::Tab4 => self.render_tab3(area, buf),
        }
    }
}

impl SelectedTab {
    /// Return tab's name as a styled `Line`
    pub fn title(&self) -> Line<'static> {
        format!("  {self}  ")
            .fg(tailwind::SLATE.c200)
            .bg(self.palette().c900)
            .into()
    }

    fn render_tab0(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Hello, World!")
            .block(self.block())
            .render(area, buf)
        // table::main::ui(f, app)
    }

    fn render_tab1(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Welcome to the Ratatui tabs example!")
            .block(self.block())
            .render(area, buf)
    }

    fn render_tab2(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Look! I'm different than others!")
            .block(self.block())
            .render(area, buf)
    }

    fn render_tab3(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("I know, these are some basic changes. But I think you got the main idea.")
            .block(self.block())
            .render(area, buf)
    }

    /// A block surrounding the tab's content
    fn block(&self) -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_set(symbols::border::PROPORTIONAL_TALL)
            .padding(Padding::horizontal(1))
            .border_style(self.palette().c700)
    }

    pub fn palette(&self) -> tailwind::Palette {
        match self {
            SelectedTab::Tab1 => tailwind::BLUE,
            SelectedTab::Tab2 => tailwind::EMERALD,
            SelectedTab::Tab3 => tailwind::INDIGO,
            SelectedTab::Tab4 => tailwind::RED,
        }
    }
}