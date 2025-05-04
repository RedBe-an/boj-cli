use reqwest::header::ACCEPT;
use serde::Deserialize;
use once_cell::sync::Lazy;
use crate::api::test_case::TestCase;
use crate::api::level::Level;
use thirtyfour::prelude::*;

static CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .build()
        .expect("failed to build HTTP client")
});

#[derive(Debug)]
pub struct Problem {
    pub id : usize,
    pub title : String,
    pub description : String,
    pub input_desc : String,
    pub output_desc : String,
    pub test_cases : Vec<TestCase>,
    pub level : Level,
}

#[derive(Debug, Deserialize)]
struct ApiProblem {
    #[serde(rename = "titleKo")] // API 제공 필드 이름
    title_ko: String,
    level: u8,
}

impl Problem {
    pub fn new(id: usize, title: String, description: String, input_desc: String, output_desc: String, level: Level ) -> Self {
        Problem {
            id,
            title,
            description,
            input_desc,
            output_desc,
            test_cases: Vec::new(),
            level,
        }
    }

    pub fn add_test_case(&mut self, input: String, output: String) {
        let test_case = TestCase { input, output };
        self.test_cases.push(test_case);
    }

    pub async fn fetch(problem_id: usize) -> anyhow::Result<Self> {
        // 1) Solved.ac API 로 기본 정보 가져오기
        let (title, level) = Self::fetch_problem_metadata(problem_id).await?;
        
        // 2) Baekjoon 문제 페이지 스크래핑
        let driver = Self::initialize_webdriver().await?;
        driver.get(format!("https://www.acmicpc.net/problem/{}", problem_id)).await?;
        
        // 3) 입력·출력 설명 스크래핑
        let (description, input_desc, output_desc) = Self::scrape_problem_description(&driver).await?;

        // 4) 문제 객체 생성
        let mut problem = Problem::new(problem_id, title, description, input_desc, output_desc, level);
        
        // 5) 예제 입출력 스크래핑
        Self::scrape_test_cases(&driver, &mut problem).await?;

        driver.quit().await?;
        Ok(problem)
    }

    async fn fetch_problem_metadata(problem_id: usize) -> anyhow::Result<(String, Level)> {
        let url = format!("https://solved.ac/api/v3/problem/show?problemId={}", problem_id);
        let api: ApiProblem = CLIENT
            .get(&url)
            .header(ACCEPT, "application/json")
            .header("x-solvedac-language", "ko")
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        
        let level = Level::from_rank(api.level as usize);
        let title = api.title_ko;
        
        Ok((title, level))
    }

    async fn initialize_webdriver() -> anyhow::Result<WebDriver> {

        let mut caps: thirtyfour::ChromeCapabilities = DesiredCapabilities::chrome();

        caps.add_arg("--headless=new")?;
        caps.add_arg("--disable-gpu")?;
        caps.add_arg("--window-size=1920,1080")?;
        

        let driver = WebDriver::new("http://localhost:4444", caps).await?;
        Ok(driver)
    }

    async fn scrape_problem_description(driver: &WebDriver) -> anyhow::Result<(String, String, String)> {

        let description = driver
            .find(By::Css("div#problem_description p"))
            .await?
            .text()
            .await?;
        let input_desc = driver
            .find(By::Css("div#problem_input p"))
            .await?
            .text()
            .await?;
        let output_desc = driver
            .find(By::Css("div#problem_output p"))
            .await?
            .text()
            .await?;
        
        Ok((description, input_desc, output_desc))
    }

    async fn scrape_test_cases(driver: &WebDriver, problem: &mut Problem) -> anyhow::Result<()> {
        let mut idx = 1;
        loop {
            let input_id = format!("sample-input-{}", idx);
            let output_id = format!("sample-output-{}", idx);
            
            let inp_elem = driver.find(By::Id(&input_id)).await;
            let outp_elem = driver.find(By::Id(&output_id)).await;
            
            // 더 이상 샘플이 없으면 종료
            if inp_elem.is_err() || outp_elem.is_err() {
                break;
            }
            
            let inp = inp_elem?.text().await?;
            let outp = outp_elem?.text().await?;
            problem.add_test_case(inp, outp);
            idx += 1;
        }
        
        Ok(())
    }
}