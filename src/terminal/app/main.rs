
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::prelude::*;
use std::io;
use strum::IntoEnumIterator;

use ratatui::widgets::*;


use crate::terminal::app::models::*;
use crate::terminal::tabs::models::*;


impl App {
    pub fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        while self.state == AppState::Running {
            self.draw(terminal)?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        terminal.draw(|frame| frame.render_widget(self, frame.size()))?;
        Ok(())
    }

    fn handle_events(&mut self) -> Result<(), io::Error> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                use KeyCode::*;
                match key.code {
                    Char('l') | Right => self.next_tab(),
                    Char('h') | Left => self.previous_tab(),
                    Char('q') | Esc => self.quit(),
                    // just for docs
                    // Char('q') | Esc => return Ok(()),
                    // Char('j') | Down => app.next(),
                    // Char('k') | Up => app.previous(),
                    // Char('l') | Right => app.next_color(),
                    // Char('h') | Left => app.previous_color(),
                    // Char('t') | Enter => app.previous_color(),
                    // Char('b') | BackTab => app.previous_color(),
                    // and here too
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }

    pub fn previous_tab(&mut self) {
        self.selected_tab = self.selected_tab.previous();
    }

    pub fn quit(&mut self) {
        self.state = AppState::Quitting;
    }


}


impl App {
    fn render_title(&self, area: Rect, buf: &mut Buffer) {
        "CSV Explorer".bold().render(area, buf);
    }

    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = SelectedTab::iter().map(|tab| tab.title());
        let highlight_style = (Color::default(), self.selected_tab.palette().c700);
        let selected_tab_index = self.selected_tab as usize;
        Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }

    fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        Line::raw("◄ ► to change tab | Press q to quit")
            .centered()
            .render(area, buf);
    }
}


impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::*;
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, title_area] = horizontal.areas(header_area);

        self.render_title(title_area, buf);
        self.render_tabs(tabs_area, buf);
        self.selected_tab.render(inner_area, buf);
        self.render_footer(footer_area, buf);
    }
}