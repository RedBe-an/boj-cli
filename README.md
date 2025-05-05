# 🚀 boj-cli

`boj-cli`는 Baekjoon Online Judge를 터미널에서 편리하게 사용하도록 도와주는 Rust 기반 커맨드라인 인터페이스입니다.

## 📁 디렉토리 구조
- **Cargo.toml**  
  프로젝트 의존성 및 메타정보
- **src/main.rs**  
  엔트리 포인트 (`Cli`/`Commands` 정의)
- **src/api/problem.rs**  
  문제 메타데이터 및 HTML 파싱 (`Problem::fetch`)
- **src/commands/**  
  - **init.rs**: `boj init` 커맨드  
  - **login.rs**: `boj login` 커맨드  
  - **run.rs**: `boj run <문제ID>` 커맨드  
  - **add.rs**: `boj add <문제ID>` 커맨드  
- **src/templates/**  
  언어별 기본 코드 템플릿
- **src/driver/**  
  Selenium WebDriver 바이너리
- **src/utils/**  
  공통 유틸 함수

## 🔧 주요 기능
1. **프로젝트 초기화** (`boj init`)  
   - `~/.boj-cli/config.toml` 생성  
   - 언어별 템플릿 복사 및 기본 설정 등록  
2. **로그인** (`boj login`)  
   - Baekjoon 사이트 인증 후 세션 쿠키 저장  
3. **문제 추가** (`boj add <문제ID>`)  
   - 웹에서 제목·설명·입출력 예제 파싱  
   - `problems/<ID>` 디렉토리 생성 및 설명 파일 작성  
4. **문제 실행** (`boj run <문제ID>`)  
   - 소스 파일 컴파일 → 실행 → 테스트 케이스 자동 적용  
   - 외부 프로세스 제어(`execute_command`)

## 📦 사용 기술
- Rust  
- Async: `tokio`  
- CLI 파싱: `clap`  
- HTML 파싱: `scraper`  
- 템플릿 포함: `include_dir`

## 🎉 빠른 시작
```bash
boj init
boj login
boj add 1000
boj run 1000
```

즐거운 코딩 되세요! 😊