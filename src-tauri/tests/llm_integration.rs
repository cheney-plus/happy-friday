use std::sync::Arc;

use futures_util::StreamExt;
use reqwest::Client;
use serde_json::{json, Value};

#[tokio::test]
async fn test_llm_streaming_chat() {
    let model = get_test_model();
    if model.is_none() {
        println!("SKIP: No test model configured. Set TEST_MODEL_JSON env var to run LLM tests.");
        return;
    }
    let model = model.unwrap();

    let url = format!("{}/chat/completions", model.base_url.trim_end_matches('/'));

    let body = json!({
        "model": model.model_name,
        "messages": [
            {"role": "system", "content": "你是一个测试助手。"},
            {"role": "user", "content": "请用一句话回答：1+1等于几？"}
        ],
        "stream": true
    });

    let client = Client::new();
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", model.api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await;

    match response {
        Ok(resp) => {
            assert!(
                resp.status().is_success(),
                "API returned status: {} - body: {:?}",
                resp.status(),
                resp.text().await.unwrap_or_default()
            );

            let mut stream = resp.bytes_stream();
            let mut buffer = String::new();
            let mut chunk_count = 0u32;
            let mut received_content = String::new();
            let mut got_done = false;

            while let Some(chunk_result) = stream.next().await {
                let chunk = chunk_result.expect("Failed to read stream chunk");
                buffer.push_str(&String::from_utf8_lossy(&chunk));

                while let Some(newline_pos) = buffer.find('\n') {
                    let line = buffer[..newline_pos].trim().to_string();
                    buffer = buffer[newline_pos + 1..].to_string();

                    if line.is_empty() {
                        continue;
                    }

                    if let Some(data) = line.strip_prefix("data: ") {
                        let data = data.trim();

                        if data == "[DONE]" {
                            got_done = true;
                            break;
                        }

                        if let Ok(parsed) = serde_json::from_str::<Value>(data) {
                            if parsed.get("error").is_some() {
                                panic!("API error: {:?}", parsed["error"]);
                            }

                            if let Some(content) =
                                parsed["choices"][0]["delta"]["content"].as_str()
                            {
                                received_content.push_str(content);
                                chunk_count += 1;
                            }
                        }
                    }
                }
            }

            println!("Received {} chunks, total content length: {}", chunk_count, received_content.len());
            println!("Content: {}", received_content);

            assert!(chunk_count > 0, "Should have received at least one streaming chunk");
            assert!(!received_content.is_empty(), "Content should not be empty");
            assert!(got_done, "Stream should end with [DONE]");
        }
        Err(e) => {
            panic!("Request failed: {}", e);
        }
    }
}

#[tokio::test]
async fn test_llm_non_streaming_title_generation() {
    let model = get_test_model();
    if model.is_none() {
        println!("SKIP: No test model configured. Set TEST_MODEL_JSON env var to run LLM tests.");
        return;
    }
    let model = model.unwrap();

    let url = format!("{}/chat/completions", model.base_url.trim_end_matches('/'));

    let body = json!({
        "model": model.model_name,
        "messages": [
            {"role": "system", "content": "请用5-10个字总结以下用户问题的主题，只返回主题文字。"},
            {"role": "user", "content": "如何学习Rust编程语言？"}
        ],
        "stream": false,
        "max_tokens": 50
    });

    let client = Client::new();
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", model.api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await;

    match response {
        Ok(resp) => {
            assert!(
                resp.status().is_success(),
                "API returned status: {}",
                resp.status()
            );

            let parsed: Value = resp.json().await.expect("Failed to parse JSON response");

            let title = parsed["choices"][0]["message"]["content"]
                .as_str()
                .unwrap_or("")
                .to_string();

            println!("Generated title: {}", title);
            assert!(!title.is_empty(), "Title should not be empty");
        }
        Err(e) => {
            panic!("Request failed: {}", e);
        }
    }
}

#[tokio::test]
async fn test_llm_error_handling() {
    let model = get_test_model();
    if model.is_none() {
        println!("SKIP: No test model configured.");
        return;
    }
    let model = model.unwrap();

    let url = format!("{}/chat/completions", model.base_url.trim_end_matches('/'));

    let body = json!({
        "model": "non-existent-model-name-12345",
        "messages": [{"role": "user", "content": "test"}],
        "stream": true
    });

    let client = Client::new();
    let result = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", model.api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await;

    match result {
        Ok(resp) => {
            println!("Error response status: {}", resp.status());
            assert!(
                !resp.status().is_success(),
                "Invalid model name should return error"
            );
        }
        Err(e) => {
            println!("Request error (expected): {}", e);
        }
    }
}

#[tokio::test]
async fn test_llm_multi_turn_conversation() {
    let model = get_test_model();
    if model.is_none() {
        println!("SKIP: No test model configured.");
        return;
    }
    let model = model.unwrap();

    let url = format!("{}/chat/completions", model.base_url.trim_end_matches('/'));

    let body = json!({
        "model": model.model_name,
        "messages": [
            {"role": "system", "content": "你是Friday助手。"},
            {"role": "user", "content": "我叫小明，记住我的名字。"},
            {"role": "assistant", "content": "好的，小明！我已经记住了你的名字。"},
            {"role": "user", "content": "我叫什么名字？"}
        ],
        "stream": true,
        "max_tokens": 100
    });

    let client = Client::new();
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", model.api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .expect("Request failed");

    assert!(response.status().is_success(), "Status: {}", response.status());

    let mut stream = response.bytes_stream();
    let mut buffer = String::new();
    let mut content = String::new();

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result.expect("Chunk error");
        buffer.push_str(&String::from_utf8_lossy(&chunk));

        while let Some(pos) = buffer.find('\n') {
            let line = buffer[..pos].trim().to_string();
            buffer = buffer[pos + 1..].to_string();

            if let Some(data) = line.strip_prefix("data: ") {
                if data.trim() == "[DONE]" {
                    break;
                }
                if let Ok(parsed) = serde_json::from_str::<Value>(data) {
                    if let Some(c) = parsed["choices"][0]["delta"]["content"].as_str() {
                        content.push_str(c);
                    }
                }
            }
        }
    }

    println!("Multi-turn response: {}", content);
    assert!(!content.is_empty(), "Response should not be empty");
}

struct TestModelConfig {
    id: String,
    provider: String,
    provider_label: String,
    api_key: String,
    model_name: String,
    base_url: String,
}

fn get_test_model() -> Option<TestModelConfig> {
    std::env::var("TEST_MODEL_JSON").ok().and_then(|json_str| {
        let v: Value = serde_json::from_str(&json_str).ok()?;
        Some(TestModelConfig {
            id: v.get("id")?.as_str()?.to_string(),
            provider: v.get("provider")?.as_str()?.to_string(),
            provider_label: v.get("providerLabel")?.as_str()?.to_string(),
            api_key: v.get("apiKey")?.as_str()?.to_string(),
            model_name: v.get("modelName")?.as_str()?.to_string(),
            base_url: v.get("baseUrl")?.as_str()?.to_string(),
        })
    })
}
