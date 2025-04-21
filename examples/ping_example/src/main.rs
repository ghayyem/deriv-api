use deriv_api::{DerivClient, Result};
use deriv_api_schema::*;
use dotenv::dotenv;
use std::env;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    // Try to load .env from different possible locations
    let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    println!("Current directory: {:?}", current_dir);
    
    // First try the current directory
    if dotenv().is_ok() {
        println!("Loaded .env file from current directory");
    } 
    // Then try the example directory
    else if dotenv::from_path("examples/ping_example/.env").is_ok() {
        println!("Loaded .env file from examples/ping_example/.env");
    } else {
        println!("Warning: Failed to load .env file");
        
        // If .env file wasn't found, set environment variables directly
        env::set_var("DERIV_API_URL", "wss://ws.binaryws.com/websockets/v3");
        env::set_var("DERIV_APP_ID", "23789");
        println!("Set environment variables manually");
    }

    // Get API URL and app_id from environment variables
    let api_url = env::var("DERIV_API_URL").unwrap_or_else(|_| {
        println!("DERIV_API_URL not set, using default");
        "wss://ws.binaryws.com/websockets/v3".to_string()
    });
    
    let app_id = env::var("DERIV_APP_ID").unwrap_or_else(|_| {
        println!("DERIV_APP_ID not set, using default");
        "23789".to_string()
    }).parse::<i32>().expect("DERIV_APP_ID must be a valid number");

    println!("Using API URL: {} and APP_ID: {}", api_url, app_id);

    // Create a new client instance.
    let mut client = DerivClient::new(
        &api_url,
        app_id,
        "en",
        "https://example.com", // Replace with your app URL or identifier
        None,
    )?;

    println!("Connecting to Deriv API...");
    // Connect to the WebSocket endpoint.
    client.connect().await?;
    println!("Connected successfully.");

    let ping_request = PingRequest {
        passthrough: None,
        ping: 1,
        req_id: None,
    };

    println!("Sending Ping request...");
    // Send the ping request using the specialized ping method
    let ping_response = client.ping(ping_request).await?;

    // Extract and print specific fields from the response object
    println!("\nPingResponse Details:");
    println!("--------------------");
    println!("msg_type: {}", ping_response.msg_type);
    
    // The ping field contains the actual "pong" response
    if let Some(ref pong) = ping_response.ping {
        println!("ping: {}", pong);
    } else {
        println!("ping: <none>");
    }
    
    // Print request ID if available
    if let Some(req_id) = ping_response.req_id {
        println!("req_id: {}", req_id);
    } else {
        println!("req_id: <none>");
    }
    
    // Print echo_req (the original request that was echoed back)
    println!("echo_req: {}", ping_response.echo_req);
    
    // You can also still print the entire response for reference
    println!("\nFull response object: {:?}", ping_response);

    // Disconnect the client (optional, happens on drop anyway).
    client.disconnect().await;
    println!("Disconnected.");

    Ok(())
}
