pub mod api;

use crate::api::problem::Problem;

#[tokio::main]
async fn main() {
    let problem_id = 2138;
    let problem = Problem::fetch(problem_id).await;
    match problem {
        Ok(problem) => {
            println!("{:?}", problem);
        }
        Err(e) => {
            eprintln!("Error fetching problem: {}", e);
        }
    }
}
