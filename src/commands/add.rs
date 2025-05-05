use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self};

use thiserror::Error;
use crate::config::Config;
use crate::api::problem::Problem;
use crate::templates::TEMPLATES;
use crate::driver::DRIVER_FILES;

#[derive(Debug, Error)]
pub enum AddError {
    #[error("Please run `boj init` to initialize the current directory.")]
    NotInitialized,
    #[error("Problem directory already exists. Use --force to overwrite.")]
    DirectoryAlreadyExists,
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error("Fetch error: {0}")]
    FetchError(String),
    #[error("Config error: {0}")]
    ConfigError(String),
}

type Result<T> = std::result::Result<T, AddError>;

pub async fn add(problem_id: u32, force: bool, extension_arg: String) -> Result<()> {
    check_initialization()?;

    let problem_dir = ensure_problem_directory(problem_id, force)?;
    let problem = fetch_problem(problem_id).await?;

    let config = Config::load().map_err(|e| AddError::ConfigError(e.to_string()))?;
    let extension = determine_extension(&extension_arg, &config);

    setup_problem_directory(&problem_dir, &problem, &extension)?;

    println!("Successfully added problem {}", problem_id);
    Ok(())
}

fn check_initialization() -> Result<()> {
    if !is_initialized() {
        return Err(AddError::NotInitialized);
    }
    Ok(())
}

fn ensure_problem_directory(problem_id: u32, force: bool) -> Result<PathBuf> {
    let problem_dir = format!("problems/{}", problem_id);
    let path = Path::new(&problem_dir);

    if path.exists() && !force {
        return Err(AddError::DirectoryAlreadyExists);
    }

    Ok(PathBuf::from(problem_dir))
}

async fn fetch_problem(problem_id: u32) -> Result<Problem> {
    Problem::fetch(problem_id)
        .await
        .map_err(|e| AddError::FetchError(e.to_string()))
}

fn determine_extension(extension_arg: &str, config: &Config) -> String {
    if extension_arg == "nil" {
        let default_ext = config.default_extension();
        println!("Using default filetype: {}", default_ext);
        default_ext.to_string()
    } else {
        extension_arg.to_string()
    }
}

fn setup_problem_directory(problem_dir: &PathBuf, problem: &Problem, extension: &str) -> Result<()> {
    fs::create_dir_all(problem_dir)?;
    fs::create_dir_all(problem_dir.join("testcases"))?;

    create_source_file(problem_dir, extension)?;
    create_description_file(problem_dir, problem)?;
    create_testcase_files(problem_dir, problem)?;

    extract_chromedriver()?;

    Ok(())
}

fn create_source_file(problem_dir: &PathBuf, extension: &str) -> Result<()> {
    // templates 폴더 안의 상대 경로
    let file_name = if extension == "java" {
        "Main.java"
    } else {
        &format!("default.{}", extension)
    };

    // include_dir 로 포함된 파일 찾기
    let file = TEMPLATES
        .get_file(file_name)
        .ok_or_else(|| AddError::ConfigError(format!("템플릿 파일이 없습니다: {}", file_name)))?;

    let contents = file
        .contents_utf8()
        .ok_or_else(|| AddError::ConfigError(format!("{} 파일이 UTF-8이 아닙니다", file_name)))?;

    let target = problem_dir.join(format!("main.{}", extension));
    fs::write(&target, contents).map_err(AddError::IoError)
}

pub fn is_initialized() -> bool {
    Path::new(".boj").exists()
}

fn create_description_file(problem_dir: &PathBuf, problem: &Problem) -> Result<()> {
    let description_file = problem_dir.join(format!("{}.md", problem.id));
    let content = format!(
        "# {}\n\n## 문제 설명 \n{}\n\n## 입력\n{}\n\n## 출력\n{}\n\n## 예제 입력\n```\n{}```\n\n## 예제 출력\n```\n{}```\n",
        problem.title, problem.description, problem.input_desc, problem.output_desc, 
        problem.test_cases[0].input, problem.test_cases[0].output
    );

    fs::write(description_file, content)?;
    Ok(())
}

fn create_testcase_files(problem_dir: &PathBuf, problem: &Problem) -> Result<()> {
    for (i, test_case) in problem.test_cases.iter().enumerate() {
        let test_case_dir = problem_dir.join("testcases").join(format!("{}", i + 1));
        fs::create_dir_all(&test_case_dir)?;

        fs::write(test_case_dir.join("input.txt"), &test_case.input)?;
        fs::write(test_case_dir.join("output.txt"), &test_case.output)?;
    }

    Ok(())
}

fn extract_chromedriver() -> Result<std::path::PathBuf> {
    // DRIVER_FILES 에 포함된 chromedriver.exe 가져오기
    let file = DRIVER_FILES
        .get_file("chromedriver.exe")
        .ok_or_else(|| AddError::ConfigError("chromedriver.exe 파일이 없습니다".into()))?;

    let bytes = file.contents();
    let out_dir = std::path::PathBuf::from(".boj/bin");
    std::fs::create_dir_all(&out_dir)?;
    let target = out_dir.join("chromedriver.exe");
    std::fs::write(&target, bytes).map_err(AddError::IoError)?;
    Ok(target)
}