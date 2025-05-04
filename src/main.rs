pub mod api;
pub mod commands;
pub mod utils;

use crate::commands::{init, login, run};
use clap::{Parser, Subcommand};
use commands::add;

#[derive(Parser)]
#[command(name = "boj-cli")]
#[command(about = "BOJ CLI tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 프로젝트 초기화
    Init {},
    /// 로그인
    Login {},
    /// 문제 실행 (예: boj-cli run 1000)
    Run {
        /// 문제 ID
        problem_id: u32,
    },
    Add {
        /// 문제 ID
        problem_id: u32,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init {} => init::init(),
        Commands::Login {} => login::login().await,
        Commands::Run { problem_id } => run::run(problem_id).await,
        Commands::Add { problem_id } => add::add(problem_id).await,
    }
}
