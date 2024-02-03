use strum::{Display, EnumIter, FromRepr};


#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum SelectedTab {
    #[default]
    #[strum(to_string = "Data Explorer")]
    Tab1,
    #[strum(to_string = "Visualization")]
    Tab2,
    #[strum(to_string = "Statistics")]
    Tab3,
    #[strum(to_string = "Extras")]
    Tab4,
}