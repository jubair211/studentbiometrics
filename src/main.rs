use clap::{Parser, Subcommand};
mod models;
mod analysis;
mod report;
mod storage;

#[derive(Parser)]
#[command(name = "studentbiometrics")]
#[command(about = "A Health Informatics CLI for student lifestyle metrics")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Log {
        #[arg(long)]
        sleep: f32,
        #[arg(long)]
        study: f32,
        #[arg(long)]
        breaks: f32,
        #[arg(long)]
        stress: u8,
        #[arg(long)]
        shift: f32,
        #[arg(long, value_name = "SCORE")]
        productivity: Option<f32>,
    },
    Analyze {
        file: String,
    },
    WeeklyReport {},
    Trends {},
    Export {
        #[arg(long)]
        format: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Log { sleep, study, breaks, stress, shift, productivity } => {
            storage::log_entry(sleep, study, breaks, stress, shift, productivity);
        }
        Commands::Analyze { file } => {
            analysis::analyze_file(&file);
        }
        Commands::WeeklyReport {} => {
            report::weekly_report();
        }
        Commands::Trends {} => {
            report::trends();
        }
        Commands::Export { format } => {
            report::export(&format);
        }
    }
}
