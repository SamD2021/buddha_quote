use serde_json::Value;

fn get_http_response(url: &str) -> Result<String, reqwest::Error> {
    // Make the HTTP request
    let body = reqwest::blocking::get(url)?.text()?;

    Ok(body)
}
fn get_api_info() -> Result<Value, Box<dyn std::error::Error>> {
    let url = "http://buddha-api.com/api/random";

    let response_text = get_http_response(url)?;

    let data: Value = serde_json::from_str(&response_text)?;
    Ok(data)
}
fn main() {
    match get_api_info() {
        Ok(data) => {
            match (
                data.get("text").and_then(|value| value.as_str()),
                data.get("byName").and_then(|value| value.as_str()),
            ) {
                (Some(text), Some(name)) => {
                    println!("> \"{}\"- {}", text, name);
                }
                _ => {
                    eprintln!("Unexpected JSON structure. Missing or invalid keys.");
                }
            }
        }
        Err(err) => {
            eprintln!("Getting the API info failed: {}", err);
        }
    }
}
