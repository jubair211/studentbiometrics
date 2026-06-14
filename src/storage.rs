use chrono::Local;
use csv::WriterBuilder;
use std::fs::{OpenOptions, create_dir_all, File};
use std::io::BufReader;
use crate::models::DailyEntry;

const DATA_FILE: &str = "data/data.csv";

pub fn log_entry(
    sleep: f32,
    study: f32,
    breaks: f32,
    stress: u8,
    shift: f32,
    productivity: Option<f32>,
) {
    create_dir_all("data").expect("Could not create data directory");
    let file_exists = std::path::Path::new(DATA_FILE).exists();
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(DATA_FILE)
        .expect("Could not open data.csv");

    let mut wtr = WriterBuilder::new()
        .has_headers(!file_exists)
        .from_writer(file);

    let entry = DailyEntry {
        date: Local::now().date_naive(),
        sleep_hours: sleep,
        study_hours: study,
        breaks_hours: breaks,
        stress_level: stress,
        shift_hours: shift,
        productivity,
    };
    wtr.serialize(entry).expect("Failed to write CSV row");
    wtr.flush().expect("Failed to flush CSV");
    println!("Entry logged successfully!");
}

pub fn load_entries() -> Vec<DailyEntry> {
    let file = File::open(DATA_FILE).expect("No data found. Log some entries first!");
    let reader = BufReader::new(file);
    let mut csv_reader = csv::Reader::from_reader(reader);
    csv_reader.deserialize()
        .map(|r| r.expect("Could not parse entry"))
        .collect()
}
