use style::palette::tailwind;
use ratatui::{prelude::*, widgets::*};
use crate::terminal::table::utils::*;
use csv::{ReaderBuilder, StringRecord};
use std::fs::File;

const PALETTES: [tailwind::Palette; 4] = [
    tailwind::BLUE,
    tailwind::EMERALD,
    tailwind::INDIGO,
    tailwind::RED,
];

const ITEM_HEIGHT: usize = 4;

pub struct TableColors {
    pub buffer_bg: Color,
    pub header_bg: Color,
    pub header_fg: Color,
    pub row_fg: Color,
    pub selected_style_fg: Color,
    pub normal_row_color: Color,
    pub alt_row_color: Color,
    pub footer_border_color: Color,
}

impl TableColors {
    pub fn new(color: &tailwind::Palette) -> Self {
        Self {
            buffer_bg: tailwind::SLATE.c950,
            header_bg: color.c900,
            header_fg: tailwind::SLATE.c200,
            row_fg: tailwind::SLATE.c200,
            selected_style_fg: color.c400,
            normal_row_color: tailwind::SLATE.c950,
            alt_row_color: tailwind::SLATE.c900,
            footer_border_color: color.c400,
        }
    }
}

pub struct App<'a> {
    // menu props
    pub menu_state: TableState,
    pub menu_items: Vec<String>,
    pub menu_scroll_state: ScrollbarState,
    pub longest_menu_item_len: u16,
    // app state
    pub app_state: TableState,
    pub items: Vec<StringRecord>,
    pub longest_item_lens: Vec<u16>, // order is (name, address, email)
    pub scroll_state: ScrollbarState,
    pub colors: TableColors,
    pub color_index: usize,
    pub table_header: Vec<String>,
    pub tab: &'a str,
}


impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        // let data_vec = generate_fake_names();
        let (headers, vals) = get_attrs();
        let menu_items = vec!["Data Explorere".to_string(), "Visualization".to_string(), "Statistics".to_string(), "Extras".to_string()];
        App {
            menu_state: TableState::default().with_selected(0),
            menu_items: menu_items.clone(),
            menu_scroll_state: ScrollbarState::new((menu_items.len() - 1) * ITEM_HEIGHT),
            longest_menu_item_len: menu_item_len_calculator(&menu_items),
            app_state: TableState::default().with_selected(0),
            longest_item_lens: constraint_len_calculator(&vals),
            scroll_state: ScrollbarState::new((vals.len() - 1) * ITEM_HEIGHT),
            colors: TableColors::new(&PALETTES[0]),
            color_index: 0,
            items: vals,
            table_header: headers,
            tab: "Data Explorer",
        }
    }

    fn get_menu_items(&self) -> Vec<&'a str> {
        let menu_items: Vec<&'a str> = vec!["Data Explorer", "Visualization", "Statistics", "Extras"];
        menu_items
    }

    pub fn next_menu(&mut self) {
        let i = match self.menu_state.selected() {
            Some(i) => {
                if i >= self.menu_items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.menu_state.select(Some(i));
        self.menu_scroll_state = self.menu_scroll_state.position(i * ITEM_HEIGHT);
        self.tab = self.get_menu_items()[i];
    }

    pub fn previous_menu(&mut self) {
        let i = match self.menu_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.menu_items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.menu_state.select(Some(i));
        self.menu_scroll_state = self.menu_scroll_state.position(i * ITEM_HEIGHT);
        self.tab = self.get_menu_items()[i]
    }

    pub fn next(&mut self) {
        let i = match self.app_state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.app_state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn previous(&mut self) {
        let i = match self.app_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.app_state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn next_color(&mut self) {
        self.color_index = (self.color_index + 1) % PALETTES.len();
    }

    pub fn previous_color(&mut self) {
        let count = PALETTES.len();
        self.color_index = (self.color_index + count - 1) % count;
    }

    pub fn set_colors(&mut self) {
        self.colors = TableColors::new(&PALETTES[self.color_index])
    }
}


fn get_attrs() -> (Vec<String>, Vec<StringRecord>) {
    let file = File::open("username.csv").expect("file does not exist");
    let mut rdr = ReaderBuilder::new().from_reader(file);
    let headers: Vec<String> = rdr.headers().expect("could not read headers from csv files").iter().map(|s| s.to_string()).collect::<Vec<String>>();
    let headers: Vec<String> = headers.iter().map(|s| s.to_string()).collect();
    println!("{:?}, {}", headers, headers.len());
    let mut records: Vec<StringRecord> = [].to_vec();
    for result in rdr.records() {
        match result {
            Ok(v) => records.push(v),
            Err(_) => continue,
        };
    };
    (headers, records)
}