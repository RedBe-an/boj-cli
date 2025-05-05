use std::process::Command;

pub async fn login() -> std::io::Result<()> {
    println!("웹 브라우저에서 BOJ 로그인 페이지를 엽니다...");

    // 브라우저 실행
    let _child = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "start", "", "https://www.acmicpc.net/login"])
            .spawn()?
    } else if cfg!(target_os = "macos") {
        Command::new("open")
            .arg("https://www.acmicpc.net/login")
            .spawn()?
    } else {
        Command::new("xdg-open")
            .arg("https://www.acmicpc.net/login")
            .spawn()?
    };

    println!("로그인 페이지가 브라우저에서 열렸습니다.");

    Ok(())
}
