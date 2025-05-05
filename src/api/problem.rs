use crate::api::level::Level;
use crate::api::test_case::TestCase;
use once_cell::sync::Lazy;
use reqwest::header::ACCEPT;
use scraper::{Html, Selector};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProblemError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("HTML parse error: {0}")]
    Parse(String),
}

static CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .build()
        .expect("failed to build HTTP client")
});

#[derive(Debug)]
pub struct Problem {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub input_desc: String,
    pub output_desc: String,
    pub test_cases: Vec<TestCase>,
    pub level: Level,
}

#[derive(Debug, Deserialize)]
struct ApiProblem {
    #[serde(rename = "titleKo")] // API 제공 필드 이름
    title_ko: String,
    level: u8,
}

impl Problem {
    pub fn new(
        id: u32,
        title: String,
        description: String,
        input_desc: String,
        output_desc: String,
        level: Level,
    ) -> Self {
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

    pub async fn fetch(problem_id: u32) -> Result<Self, ProblemError> {
        let (meta, page_html) = tokio::try_join!(
            Self::fetch_problem_metadata(problem_id),
            Self::fetch_problem_page(problem_id),
        )?;

        let (title, level) = meta;
        let document = Html::parse_document(&page_html);

        // 셀렉터를 static 으로 캐시
        static DESC_SEL: Lazy<Selector> =
            Lazy::new(|| Selector::parse("div#problem_description p").unwrap());
        static INPUT_SEL: Lazy<Selector> =
            Lazy::new(|| Selector::parse("div#problem_input p").unwrap());
        static OUTPUT_SEL: Lazy<Selector> =
            Lazy::new(|| Selector::parse("div#problem_output p").unwrap());
        static SAMPLE_INP: Lazy<Selector> = Lazy::new(|| {
            Selector::parse("pre#sample-input-1, pre[id^=\"sample-input-\"]").unwrap()
        });
        static SAMPLE_OUT: Lazy<Selector> = Lazy::new(|| {
            Selector::parse("pre#sample-output-1, pre[id^=\"sample-output-\"]").unwrap()
        });

        let description = document
            .select(&*DESC_SEL)
            .map(|e| e.text().collect::<String>())
            .next()
            .ok_or_else(|| ProblemError::Parse("Missing problem description".into()))?;
        let input_desc = document
            .select(&*INPUT_SEL)
            .map(|e| e.text().collect::<String>())
            .next()
            .ok_or_else(|| ProblemError::Parse("Missing input description".into()))?;
        let output_desc = document
            .select(&*OUTPUT_SEL)
            .map(|e| e.text().collect::<String>())
            .next()
            .ok_or_else(|| ProblemError::Parse("Missing output description".into()))?;

        let mut problem = Problem::new(
            problem_id,
            title,
            description,
            input_desc,
            output_desc,
            level,
        );

        // 예제 입출력 파싱
        for (inp, outp) in document
            .select(&*SAMPLE_INP)
            .zip(document.select(&*SAMPLE_OUT))
        {
            problem.add_test_case(
                inp.text().collect::<String>(),
                outp.text().collect::<String>(),
            );
        }

        Ok(problem)
    }

    async fn fetch_problem_metadata(problem_id: u32) -> Result<(String, Level), reqwest::Error> {
        let url = format!(
            "https://solved.ac/api/v3/problem/show?problemId={}",
            problem_id
        );
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

    async fn fetch_problem_page(problem_id: u32) -> Result<String, reqwest::Error> {
        let url = format!("https://www.acmicpc.net/problem/{}", problem_id);
        let res = CLIENT
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        Ok(res)
    }
}
