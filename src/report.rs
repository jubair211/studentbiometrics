use crate::analysis::{weekly_summary, daily_health};
use crate::storage::load_entries;

pub fn weekly_report() {
    let entries = load_entries();
    if entries.is_empty() {
        println!("No entries found. Log some data first.");
        return;
    }
    let summary = weekly_summary(&entries);
    println!("===== Weekly Report =====");
    println!("Average Sleep:     {:.1}h", summary.avg_sleep);
    println!("Total Study Hours: {:.1}h", summary.total_study);
    println!("Total Shift Hours: {:.1}h", summary.total_shift);
    println!("Average Stress:    {:.1}/5", summary.avg_stress);
    println!("Health Score:      {:.2}", summary.avg_health_score);
    println!("Trend:             {}", summary.trend_label);
}

pub fn trends() {
    let entries = load_entries();
    if entries.is_empty() {
        println!("No entries found. Log some data first.");
        return;
    }
    println!("===== Daily Health Trend =====");
    for e in &entries {
        let score = daily_health(e);
        let label = if score >= 5.0 {
            "Good"
        } else if score >= 3.0 {
            "At Risk"
        } else {
            "Burnout Risk"
        };
        println!("{} -> {:.2} ({})", e.date, score, label);
    }
}

pub fn export(format: &str) {
    let entries = load_entries();
    if entries.is_empty() {
        println!("No entries found. Log some data first.");
        return;
    }
    match format {
        "json" => {
            let json = serde_json::to_string_pretty(&entries).unwrap();
            std::fs::write("data/export.json", json).unwrap();
            println!("Exported to data/export.json");
        }
        "csv" => {
            std::fs::copy("data/data.csv", "data/export.csv").unwrap();
            println!("Exported to data/export.csv");
        }
        _ => println!("Unsupported format. Use --format json or csv"),
    }
}
