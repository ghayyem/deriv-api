use deriv_api::{DerivClient, Result};
use deriv_api_schema::*;
use futures::StreamExt;
use serde_json::Value;
use std::time::{Duration, Instant};
use dotenv::dotenv;
use std::env;
use std::path::{Path, PathBuf};

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
    else if dotenv::from_path("examples/ticks_stream_example/.env").is_ok() {
        println!("Loaded .env file from examples/ticks_stream_example/.env");
    } else {
        println!("Warning: Failed to load .env file");
        
        // If .env file wasn't found, set environment variables directly
        env::set_var("DERIV_API_URL", "wss://ws.binaryws.com/websockets/v3");
        env::set_var("DERIV_APP_ID", "23789");
        println!("Set environment variables manually");
    }
    
    // Initialize logger
    env_logger::init();

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
        "https://example.com",
        None,
    )?;

    println!("Connecting to Deriv API...");
    client.connect().await?;
    println!("Connected successfully.");

    // Symbol to stream ticks for
    let symbol = "R_50"; // Random index 50
    
    // Create a ticks subscription request
    let ticks_request = TicksRequest {
        ticks: Value::String(symbol.to_string()),
        subscribe: Some(1),
        req_id: None,
        passthrough: None,
    };

    println!("Subscribing to ticks for {}...", symbol);
    
    // Subscribe to tick updates using the new subscription mechanism
    let (initial_response, mut subscription) = client.subscribe_ticks(ticks_request).await?;
    
    // Extract values from the initial response
    if let Some(tick_value) = initial_response.tick.as_ref() {
        // Print initial tick value and subscription ID
        println!("\nInitial tick:");
        println!("Symbol: {}", tick_value.get("symbol").unwrap_or(&Value::Null).as_str().unwrap_or("Unknown"));
        println!("Quote: {}", tick_value.get("quote").unwrap_or(&Value::Null).as_f64().unwrap_or(0.0));
        println!("Epoch: {}", tick_value.get("epoch").unwrap_or(&Value::Null).as_i64().unwrap_or(0));
        println!("Subscription ID: {}", subscription.subscription_id());
    } else {
        println!("\nNo initial tick data available");
        println!("Subscription ID: {}", subscription.subscription_id());
    }
    
    println!("\nStreaming ticks for 15 seconds:");
    println!("------------------------------");
    
    // Set up timer for 15 seconds
    let start_time = Instant::now();
    let duration = Duration::from_secs(15);
    let end_time = start_time + duration;
    let mut tick_count = 0;
    
    // Create a wrapped future for subscription.next() with a timeout
    let mut interval = tokio::time::interval(Duration::from_millis(100));
    
    // Process streaming updates until the time limit is reached
    while Instant::now() < end_time {
        tokio::select! {
            // Poll for new ticks
            Some(tick_update) = subscription.next() => {
                tick_count += 1;
                
                // Extract tick data from the update
                if let Some(tick_value) = tick_update.tick.as_ref() {
                    // Print tick data
                    println!("Tick #{}: Symbol: {}, Quote: {}, Epoch: {}", 
                        tick_count,
                        tick_value.get("symbol").unwrap_or(&Value::Null).as_str().unwrap_or("Unknown"), 
                        tick_value.get("quote").unwrap_or(&Value::Null).as_f64().unwrap_or(0.0),
                        tick_value.get("epoch").unwrap_or(&Value::Null).as_i64().unwrap_or(0)
                    );
                } else {
                    println!("Tick #{}: No tick data available", tick_count);
                }
            }
            
            // Wait for the interval tick
            _ = interval.tick() => {
                // Just a timeout to keep the loop from spinning too fast
                // We're checking Instant::now() in the while condition
            }
            
            // If subscription ended or error occurred
            else => {
                println!("\nSubscription ended unexpectedly.");
                break;
            }
        }
    }
    
    println!("\nReached 15-second time limit.");
    
    // Explicitly forget the subscription when done
    println!("\nUnsubscribing from ticks...");
    subscription.forget().await?;
    println!("Unsubscribed successfully.");
    
    println!("\nStream finished after {} ticks in {:?}", tick_count, start_time.elapsed());
    
    // Disconnect
    client.disconnect().await;
    println!("Disconnected.");

    Ok(())
}