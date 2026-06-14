use crate::models::{DailyEntry, WeeklySummary};
use std::io::BufReader;
use std::fs::File;

pub fn sleep_debt(sleep: f32) -> f32 {
    (8.0 - sleep).max(0.0)
}

pub fn balance_score(entry: &DailyEntry) -> f32 {
    let debt = sleep_debt(entry.sleep_hours);
    entry.study_hours
        - (debt * 0.8)
        - (entry.stress_level as f32 * 0.5)
        - (entry.shift_hours * 0.3)
        + (entry.breaks_hours * 0.1)
}

pub fn productivity_auto(entry: &DailyEntry) -> f32 {
    let debt = sleep_debt(entry.sleep_hours);
    (entry.study_hours * 0.6)
        + (entry.breaks_hours * 0.2)
        - (entry.stress_level as f32 * 0.4)
        - (debt * 0.5)
}

pub fn daily_health(entry: &DailyEntry) -> f32 {
    let balance = balance_score(entry);
    let prod = entry.productivity.unwrap_or_else(|| productivity_auto(entry));
    balance * 0.7 + prod * 0.3
}

pub fn weekly_summary(entries: &[DailyEntry]) -> WeeklySummary {
    let n = entries.len() as f32;
    let avg_sleep = entries.iter().map(|e| e.sleep_hours).sum::<f32>() / n;
    let total_study = entries.iter().map(|e| e.study_hours).sum::<f32>();
    let total_shift = entries.iter().map(|e| e.shift_hours).sum::<f32>();
    let avg_stress = entries.iter().map(|e| e.stress_level as f32).sum::<f32>() / n;
    let avg_health_score = entries.iter().map(|e| daily_health(e)).sum::<f32>() / n;

    let trend_label = if avg_health_score >= 5.0 {
        "stable".to_string()
    } else if avg_health_score >= 3.0 {
        "at risk".to_string()
    } else {
        "burnout risk".to_string()
    };

    WeeklySummary {
        avg_sleep,
        total_study,
        total_shift,
        avg_stress,
        avg_health_score,
        trend_label,
    }
}

pub fn analyze_file(path: &str) {
    let file = File::open(path).expect("Could not open file");
    let reader = BufReader::new(file);
    let mut csv_reader = csv::Reader::from_reader(reader);

    let entries: Vec<DailyEntry> = csv_reader.deserialize()
        .map(|r| r.expect("Could not parse row"))
        .collect();

    let summary = weekly_summary(&entries);
    println!("=== Analysis Report ===");
    println!("Avg Sleep:    {:.1}h", summary.avg_sleep);
    println!("Total Study:  {:.1}h", summary.total_study);
    println!("Total Shift:  {:.1}h", summary.total_shift);
    println!("Avg Stress:   {:.1}/5", summary.avg_stress);
    println!("Health Score: {:.1}", summary.avg_health_score);
    println!("Trend:        {}", summary.trend_label);
}
