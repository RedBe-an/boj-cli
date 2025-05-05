pub mod api;
pub mod commands;
pub mod config;
pub mod driver;
pub mod templates;
pub mod utils;

use crate::commands::{init, login, run};
use clap::{Parser, Subcommand};
use commands::add;

#[derive(Parser)]
#[command(name = "boj")]
#[command(about = "baekjoon online judge commandline interface.", long_about = None)]
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
    /// 문제 추가
    Add {
        /// 문제 ID
        problem_id: u32,
        /// 이미 존재하는 파일을 덮어쓸지 여부
        #[arg(short, long)]
        force: bool,
        /// 소스 파일 확장자 (예: rs, cpp)
        #[arg(short, long, default_value = "nil")]
        extension: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result: Result<(), Box<dyn std::error::Error>> = match cli.command {
        Commands::Init {} => init::init().map_err(|e| e.into()),
        Commands::Login {} => login::login().await.map_err(|e| e.into()),
        Commands::Run { problem_id } => run::run(problem_id).await.map_err(|e| e.into()),
        Commands::Add {
            problem_id,
            force,
            extension,
        } => add::add(problem_id, force, extension)
            .await
            .map_err(|e| e.into()),
    };

    if let Err(err) = result {
        eprintln!("오류 발생: {}", err);
        std::process::exit(1);
    }
}
