use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DailyEntry {
    pub date: NaiveDate,
    pub sleep_hours: f32,
    pub study_hours: f32,
    pub breaks_hours: f32,
    pub stress_level: u8,
    pub shift_hours: f32,
    pub productivity: Option<f32>,
}

#[derive(Debug)]
pub struct WeeklySummary {
    pub avg_sleep: f32,
    pub total_study: f32,
    pub total_shift: f32,
    pub avg_stress: f32,
    pub avg_health_score: f32,
    pub trend_label: String,
}
