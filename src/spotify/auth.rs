use tiny_http::{Server, Response, Request};
use serde::{Deserialize, Serialize};
use anyhow::Result;

const REDIRECT_URI: &str = "http://localhost:8888/callback"; //perfect, now we wait for out github to be unbanned lmao
const AUTH_PORT: u16 = 8888;
const BIND_ADDRESS: &str = "0.0.0.0";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<u64>,
}

pub async fn login(account_name: &str) -> Result<()> {
    let client_id = std::env::var("SPOTIFY_CLIENT_ID")?;
    let client_secret = std::env::var("SPOTIFY_CLIENT_SECRET")?;

    // Build authorization URL
    let scopes = vec![
        "user-library-read",
        "user-library-modify", 
        "playlist-read-private",
        "playlist-modify-public",
        "playlist-modify-private",
    ];
    
    let auth_url = format!(
        "https://accounts.spotify.com/authorize?\
         client_id={}&\
         response_type=code&\
         redirect_uri={}&\
         scope={}",
        client_id,
        urlencoding::encode(REDIRECT_URI),
        urlencoding::encode(&scopes.join(" "))
    );

    // Start local server
    let server = Server::http(format!("{}:{}", BIND_ADDRESS, AUTH_PORT))
        .expect("Failed to start local server");

    // Open browser
    println!("🌐 Opening browser for authentication...");
    webbrowser::open(&auth_url)?;
    
    println!("⏳ Waiting for authorization...");

    // Thennn Wait for callback
    let request = server.recv()?;
    let code = extract_code_from_request(&request)?;

    // Send success page
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head><title>Success</title></head>
        <body style="font-family: sans-serif; text-align: center; padding: 50px;">
            <h1>✅ Success!</h1>
            <p>You can close this window now.</p>
        </body>
        </html>
    "#;
    request.respond(Response::from_string(html))?;

    // Exchange code for tokens
    let tokens = exchange_code_for_tokens(&code, &client_id, &client_secret).await?;

    // Save tokens in our local storage
    crate::storage::tokens::save_tokens(account_name, &tokens)?;

    Ok(())
}

fn extract_code_from_request(request: &Request) -> Result<String> {
    let url = request.url();
    
    // Parse URL query params
    if let Some(query_start) = url.find('?') {
        let query = &url[query_start + 1..];
        for param in query.split('&') {
            if let Some((key, value)) = param.split_once('=') {
                if key == "code" {
                    return Ok(value.to_string());
                }
            }
        }
    }
    
    anyhow::bail!("No authorization code found in callback URL")
}

async fn exchange_code_for_tokens(
    code: &str,
    client_id: &str,
    client_secret: &str,
) -> Result<Tokens> {
    let client = reqwest::Client::new();
    
    let params = [
        ("grant_type", "authorization_code"),
        ("code", code),
        ("redirect_uri", REDIRECT_URI),
    ];

    let response = client
        .post("https://accounts.spotify.com/api/token")
        .basic_auth(client_id, Some(client_secret))
        .form(&params)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        anyhow::bail!("Failed to exchange code for tokens: {}", error_text);
    }

    let mut tokens: Tokens = response.json().await?;
    
    // Calculate expiration timestamp
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();
    tokens.expires_at = Some(now + tokens.expires_in);

    Ok(tokens)
}