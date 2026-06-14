# StudentBioMetrics

A Health Informatics CLI tool for international students balancing study, sleep, stress, and shift work.

## The Problem

As an international student working shifts, it is hard to track whether your daily habits are sustainable. Are you sleeping enough? Is your stress level affecting your productivity? Are your shift hours hurting your study performance?

StudentBioMetrics answers these questions from your terminal.

## Features

- Log daily metrics: sleep, study, breaks, stress, shift hours, productivity
- Auto-calculate productivity score based on your data
- Generate weekly health reports
- Show daily health trends
- Export data as JSON or CSV
- Streams data efficiently using BufReader

## Installation

Download the latest binary from the releases page or build from source:

    cargo build --release

## Usage

Log daily data:

    studentbiometrics log --sleep 7 --study 4 --breaks 1 --stress 3 --shift 5

Log with manual productivity score:

    studentbiometrics log --sleep 7 --study 4 --breaks 1 --stress 3 --shift 5 --productivity 8

Weekly report:

    studentbiometrics weekly-report

Show daily trends:

    studentbiometrics trends

Analyze a CSV file:

    studentbiometrics analyze data.csv

Export data:

    studentbiometrics export --format json
    studentbiometrics export --format csv

## Scoring

Sleep Debt = max(0, 8 - sleep_hours)

Balance Score = study - (sleep_debt x 0.8) - (stress x 0.5) - (shift x 0.3) + (breaks x 0.1)

Health Score = balance x 0.7 + productivity x 0.3

## Target Users

- International students balancing study and shift work
- Health Informatics students
- Students at risk of burnout

## Built With

- Rust
- clap - CLI argument parsing
- chrono - date handling
- serde - serialization
- csv - efficient CSV reading with BufReader
- serde_json - JSON export

## License

MIT
