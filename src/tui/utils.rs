use unicode_width::UnicodeWidthStr;
use csv::StringRecord;


pub fn constraint_len_calculator(items: &[StringRecord]) -> Vec<u16> {
    let mut res: Vec<u16> = [].to_vec();
    for item in items {
        let vals = item.iter()
            .map(|v| UnicodeWidthStr::width(v))
            .max()
            .unwrap_or(0);

        res.push(vals as u16)
    }
    res
}

pub fn menu_item_len_calculator(items: &[String]) -> u16 {
    let name_len = items
        .iter()
        .map(|s| UnicodeWidthStr::width(s.as_str()))
        .max()
        .unwrap_or(0);

    name_len as u16
}