use actix_web::{HttpResponse, Query, Result};
use askama::Template;
use std::collections::HashMap;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    name: &'a str,
}

#[derive(Deserialize, Debug)]
struct TriviaResponse {
    response_code: u32,
    results: Vec<TriviaSpec>,
}

#[derive(Deserialize, Debug)]
struct TriviaSpec {
    category: String,
    #[serde(rename = "type")]
    ty: String,
    difficulty: String,
    question: String,
    correct_answer: String,
    incorrect_answers: Vec<String>,
}

pub fn index(query: Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let fallback_name = &"world".to_string();
    let name = query.get("name").unwrap_or(fallback_name);

    let trivia_response: TriviaResponse = reqwest::get("https://opentdb.com/api.php?amount=1")
        .unwrap()
        .json()
        .unwrap();
    println!("{:?}", trivia_response);

    let s = IndexTemplate { name: name }.render().unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
