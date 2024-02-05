use style::palette::tailwind;
use ratatui::{prelude::*, widgets::*};
use crate::tui::utils::*;
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


#[derive(Clone, Debug)]
pub struct Data {
    pub data: Vec<f64>,
    pub label: String,
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
    pub grouped_headers: Vec<(String, String)>,
    pub plot_data: Vec<Vec<(f64, f64)>>,
    pub raw_data: Vec<Data>,
    pub stats_header: Vec<String>
}

fn parse_records(records: &Vec<StringRecord>, headers: &Vec<String>) -> (Vec<(String, String)>, Vec<Vec<(f64, f64)>>, Vec<Data>) {
    let mut res: Vec<Data> = [].to_vec();
    let mut column_vectors: Vec<Vec<String>> = vec![Vec::new(); headers.len()];

    for (_, record) in records.iter().enumerate() {
        for (index, field) in record.iter().enumerate() {
            // Store the field in the corresponding column vector
            column_vectors[index].push(field.to_string());
        }
    }

    // Sort each column vector
    for column_vector in &mut column_vectors {
        column_vector.sort();
    }

     // Print sorted vectors
     let mut digit_headers: Vec<String> = Vec::new();
     for (index, column_vector) in column_vectors.iter().enumerate() {
        if is_valid_float(column_vector[0].as_str()) {
            res.push(Data { data:  parse_strings_to_floats(column_vector), label: headers.get(index).unwrap().to_string() });
            digit_headers.push(headers.get(index).unwrap().to_string());
        }
    }

    let mut fin: Vec<Vec<(f64, f64)>> = [].to_vec();
    let mut aa: Vec<f64> = Vec::new();

    if res.len() != 0 {
        if res.len() % 2 != 0 {
            let mut current_number = 0.0;
            let mut aa: Vec<f64> = Vec::new();
            while current_number <= res[0].data.len() as f64 {
                aa.push(current_number);
                current_number += 1.0;
            }
        }
    
        let mut new_vec: Vec<Vec<f64>> = [].to_vec();
        let bb = res.clone();
        for i in bb {
            new_vec.push(i.data)
        }
    
        new_vec.push(aa);
        digit_headers.push("Increasing counter".to_string());
        let grouped_vectors: Vec<Vec<Vec<f64>>> = new_vec.chunks_exact(2).map(|chunk| chunk.to_vec()).collect();
        for g in grouped_vectors {
            let merged_vector: Vec<(f64, f64)> = g[0].clone().into_iter().zip(g[1].clone().into_iter()).collect();
            fin.push(merged_vector)
        }
    }

    let grouped_headers: Vec<(String, String)> = digit_headers
        .chunks(2)
        .map(|chunk| match chunk {
            [a, b] => (a.clone(), b.clone()),
            [a] => (a.clone(), String::new()), // Handle case with odd number of elements
            _ => unreachable!(),
        })
        .collect();
    
 
    return (grouped_headers, fin, res)
}

fn is_valid_float(s: &str) -> bool {
    s.trim().parse::<f64>().is_ok()
}

fn parse_strings_to_floats(strings: &Vec<String>) -> Vec<f64> {
    strings
        .iter()
        .filter_map(|s| s.parse::<f64>().ok())
        .collect()
}

impl<'a> App<'a> {
    pub fn new(file_path: String) -> App<'a> {
        // let data_vec = generate_fake_names();
        let mut stats_headers = vec![
            "S/N".to_string(), "Measurement".to_string(),
        ];
        let (headers, vals) = get_attrs(file_path);
        let menu_items = vec!["Data Explorere".to_string(), "Visualization".to_string(), "Statistics".to_string(), "Extras".to_string()];
        let (grouped_headers, plot_data, raw_data) = parse_records(&vals, &headers);
        for d in raw_data.clone() {
            stats_headers.push(d.label)
        }
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
            items: vals.clone(),
            table_header: headers.clone(),
            tab: "Data Explorer",
            grouped_headers,
            raw_data,
            plot_data,
            stats_header: stats_headers,
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


fn get_attrs(file_path: String) -> (Vec<String>, Vec<StringRecord>) {
    let file = File::open(file_path).expect("provided file does not exist");
    let mut rdr = ReaderBuilder::new().from_reader(file);
    let headers: Vec<String> = rdr.headers().expect("could not read headers from csv files").iter().map(|s| s.to_string()).collect::<Vec<String>>();
    let headers: Vec<String> = headers.iter().map(|s| s.to_string()).collect();
    // println!("{:?}, {}", headers, headers.len());
    let mut records: Vec<StringRecord> = [].to_vec();
    for result in rdr.records() {
        match result {
            Ok(v) => records.push(v),
            Err(_) => continue,
        };
    };
    (headers, records)
}