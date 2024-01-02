mod ai_functions;
mod apis;
mod helpers;
mod models;

use helpers::cli::get_user_response;
use helpers::questions::Questions;

fn main() {
    let questions = Questions::new();
    let user_request = get_user_response(&questions.initial);

    dbg!(user_request);
}
