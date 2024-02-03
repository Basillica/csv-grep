use crate::terminal::tabs::models::*;

#[derive(Default)]
pub struct App {
    pub state: AppState,
    pub selected_tab: SelectedTab,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    #[default]
    Running,
    Quitting,
}