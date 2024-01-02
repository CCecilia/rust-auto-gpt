use crate::models::general::llm::{ChatCompletion, Message, APIResponse};
use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::ClientBuilder;
use std::env;
use std::error;

pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn error::Error + Send>> {
    dotenv().ok();

    // let api_key = env::var("OPEN_AI_KEY").expect("Open AI key not found").;
    let api_key = match env::var("OPEN_AI_KEY") {
        Ok(key) => key,
        Err(err) => {
            eprintln!("{}", err);
            panic!("invalid environment")
        }
    };
    let bearer_token = format!("Bearer {}", api_key);
    // let org_id = env::var("OPEN_AI_ORG").expect("Open AI org id not found");
    let url: &str = "https://api.openai.com/v1/chat/completions";
    let gpt_models = vec!["gpt-3.5-turbo", "gpt-4"];

    let client = ClientBuilder::new()
        .default_headers({
            let mut headers = HeaderMap::new();

            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(bearer_token.as_str()).map_err(|err| ->  Box<dyn error::Error + Send> {Box::new(err)})?,
            );
            headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_str("application/json").map_err(|err| ->  Box<dyn error::Error + Send> {Box::new(err)})?,
            );

            dbg!(&headers);

            headers
        })
        .build()
        .map_err(|err| ->  Box<dyn error::Error + Send> {Box::new(err)})?;

    let chat_completion = ChatCompletion {
        model: gpt_models[0].to_string(),
        messages,
        temperature: 0.1
    };

    //Debugging
    // let response = client.post(url).json(&chat_completion).send().await.unwrap();
    // dbg!(response.text().await.unwrap());

    let res: APIResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // Ok("some test string".to_string())
    Ok(res.choices[0].message.content.clone()) //TODO: Fix this cloning
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_call_to_openai() {
        let messages = vec![Message {
            role: "user".to_string(),
            content: "This is a test please give me short response".to_string()
        }] ;

        match call_gpt(messages).await {
            Ok(response_content) => {
                dbg!(response_content);
                assert!(true)
            },
            Err(_) => assert!(false)
        }

        // let response = call_gpt(messages).await;

        // if let OK(response_content) = response {
        //     assert(true)
        // } else {
        //     assert!(false)
        // }

    }
}
