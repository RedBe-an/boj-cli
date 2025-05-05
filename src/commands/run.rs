use crate::config::Config;
use std::io::Write;
use std::process::{Command, Stdio};
use std::error::Error;

pub async fn run(problem_id: u32) -> Result<(), Box<dyn std::error::Error>> {
    let problem_dir = format!("problems/{}", problem_id);
    let testcases_dir = format!("{}/testcases", problem_dir);
    // testcases_dir에 있는 폴더의 개수
    let testcases = std::fs::read_dir(testcases_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
        .collect::<Vec<_>>();

    println!("Found {} testcases", testcases.len());

    for testcase in testcases {
        let testcase_name = testcase.file_name().into_string().unwrap();
        let input_path = format!("{}/testcases/{}/input.txt", problem_dir, testcase_name);
        let output_path = format!("{}/testcases/{}/output.txt", problem_dir, testcase_name);

        // Read the input and expected output files
        let input = std::fs::read_to_string(input_path)?;
        let expected_output = std::fs::read_to_string(output_path)?;
        // Windows CRLF → LF 변환 및 양끝 공백/개행 제거
        let expected_output = expected_output.replace("\r\n", "\n").trim().to_string();

        // Run the solution with the input
        let output = run_solution(&problem_id, &input).await?;
        // 실행 결과(예: child.stdout)가 담긴 변수 `output` 에도 동일 처리
        let output = output.replace("\r\n", "\n").trim().to_string();

        // Compare the output with the expected output
        if output == expected_output {
            println!("Testcase {} passed", testcase_name);
        } else {
            println!("Testcase {} failed", testcase_name);
            println!("Expected: {}", expected_output);
            println!("Got: {}", output);
        }
    }
    Ok(())
}

pub async fn run_solution(problem_id: &u32, input: &str) -> Result<String, Box<dyn Error>> {
    // 1. 설정 로드
    let config = Config::load()?;
    let problem_dir = format!("problems/{}", problem_id);

    // 2. main 파일을 가진 파일타입 섹션 찾기
    let (_ext, ft) = config.filetype.iter()
        .find(|(_, ft)| std::path::Path::new(&problem_dir).join(&ft.main).exists())
        .ok_or("no matching filetype section")?;

    // 3. 컴파일 (있다면)
    if let Some(compile_cmd) = &ft.compile {
        execute_command(compile_cmd, &problem_dir, None)?;
    }

    // 4. 실행
    let output = execute_command(&ft.run, &problem_dir, Some(input))?;

    // 5. after (있다면)
    if let Some(after_cmd) = &ft.after {
        let _ = execute_command(after_cmd, &problem_dir, None)?;
    }

    Ok(output)
}

// helper: 커맨드를 실행하고 stdout 리턴
fn execute_command(cmd: &str, workdir: &str, input: Option<&str>) -> Result<String, Box<dyn Error>> {
    // 1. workdir 을 절대 경로로 변환
    let abs_workdir = std::fs::canonicalize(workdir)?;

    // 2. 명령어 파싱
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    let mut c = Command::new(parts[0]);
    if parts.len() > 1 {
        c.args(&parts[1..]);
    }

    // 3. 해당 문제 폴더를 CWD 로 설정하고 실행
    c.current_dir(abs_workdir)
     .stdin(if input.is_some() { Stdio::piped() } else { Stdio::null() })
     .stdout(Stdio::piped())
     .stderr(Stdio::inherit());

    let mut child = c.spawn()?;

    if let Some(stdin_data) = input {
        child.stdin.as_mut().unwrap().write_all(stdin_data.as_bytes())?;
    }

    let output = child.wait_with_output()?;
    if !output.status.success() {
        return Err(format!("command failed: {}", cmd).into());
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}