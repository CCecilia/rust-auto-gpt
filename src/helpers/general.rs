use crate::{models::general::llm::Message, apis::call_request::call_gpt};
use serde::de::DeserializeOwned;
use reqwest::Client;
use std::fs;

use super::cli::PrintCommand;

const CODE_TEMPLATE_PATH: &str =
    "/home/doughboy/Projects/rustProjects/Tuts/web_template/src/server_template.rs";

pub const WEB_SERVER_PROJECT_PATH: &str = "/home/doughboy/Projects/rustProjects/Tuts/web_template/";

pub const EXEC_MAIN_PATH: &str =
    "/home/doughboy/Projects/rustProjects/Tuts/web_template/src/main.rs";

const API_SCHEMA_PATH: &str =
    "/home/doughboy/Projects/rustProjects/Tuts/auto_gpt/schemas/api_schema.json";

pub fn extend_ai_functions(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_function_str = ai_func(func_input);


    let msg = format!("FUNCTION {}
    INSTRUCTION: You are a function printer. You ONLY print the results of functions with NO commentary. Here is the function input: {}
    Please print out what the function will return.
    ", ai_function_str, func_input);

    // dbg!(msg);

    Message {
        role: "system".to_string(),
        content: msg
    }
}

// Performs call to LLM GPT
pub async fn ai_task_request(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str, //using generic lifetime to ensure function string doesnt get deref'd forthe lifetime of the program
) -> String {
    let extended_msg = extend_ai_functions(function_pass, &msg_context);

    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

    let llm_response = call_gpt(vec![extended_msg.clone()]).await;

    match llm_response {
        Ok(response_text) => response_text,
        Err(err) => {
            eprintln!("First failure to call openai | {}", err);
            call_gpt(vec![extended_msg.clone()]).await.expect("Openai unreachable")
        }
    }
}

pub async fn ai_task_request_decoded<T: DeserializeOwned>(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> T {
    let llm_response: String =
        ai_task_request(msg_context, agent_position, agent_operation, function_pass).await;
    let decoded_response: T = serde_json::from_str(llm_response.as_str())
        .expect("Failed to decode ai response from serde_json");

    decoded_response
}

// Check whether request url is valid
pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let response: reqwest::Response = client.get(url).send().await?;
    Ok(response.status().as_u16())
}

// Get Code Template
pub fn read_code_template_contents() -> String {
    let path: String = String::from(CODE_TEMPLATE_PATH);
    fs::read_to_string(path).expect("Failed to read code template")
}

// Get Exec Main
pub fn read_exec_main_contents() -> String {
    let path: String = String::from(EXEC_MAIN_PATH);
    fs::read_to_string(path).expect("Failed to read code template")
}

// Save New Backend Code
pub fn save_backend_code(contents: &String) {
    let path: String = String::from(EXEC_MAIN_PATH);
    fs::write(path, contents).expect("Failed to write main.rs file");
}

// Save JSON API Endpoint Schema
pub fn save_api_endpoints(api_endpoints: &String) {
    let path: String = String::from(API_SCHEMA_PATH);
    fs::write(path, api_endpoints).expect("Failed to write API Endpoints to file");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn test_extending_ai_function() {
        let message = extend_ai_functions(convert_user_input_to_goal,"dummy variable");

        assert_eq!(message.role, "system".to_string());
        assert_eq!(message.content.starts_with("FUNCTION"), true)
    }

    #[tokio::test]
    async fn tests_ai_task_request() {
        let ai_func_param: String =
            "Build me a webserver for making stock price api requests.".to_string();

        let res: String = ai_task_request(
            ai_func_param,
            "Managing Agent",
            "Defining user requirements",
            convert_user_input_to_goal,
        )
        .await;

        assert!(res.len() > 20);
    }
}