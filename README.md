# Deriv API Rust Client

[![Crates.io](https://img.shields.io/crates/v/deriv-api.svg)](https://crates.io/crates/deriv-api)
[![Documentation](https://docs.rs/deriv-api/badge.svg)](https://docs.rs/deriv-api)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

**An unofficial Rust library for the [Deriv API](https://api.deriv.com)**

This crate provides a strongly-typed client for the Deriv API, with automatically generated types from the official API schema. It supports both synchronous and asynchronous WebSocket connections, allowing you to interact with your Deriv account programmatically.

## Features

- 🔄 Automatic code generation from official API schema
- 🔐 Type-safe API requests and responses
- 📈 Real-time market data streaming
- 🚀 Async/await support
- 🛡️ Comprehensive error handling

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
deriv-api = "0.1.0"
```

## Quick Start

### Subscribing to Tick Stream

```rust
use deriv_api::{DerivClient, Result};
use deriv_api_schema::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Create a new client
    let client = DerivClient::new(
        "wss://ws.binaryws.com/websockets/v3",
        1234, // Your app_id
        "en",
        "https://your-app.com",
        None,
    )?;

    // Connect to the WebSocket
    client.connect().await?;

    // Create tick request
    let request = TicksRequest {
        ticks: Value::String("R_50".to_string()),
        subscribe: Some(SubscribeEnum::Value1),
        ..Default::default()
    };

    // Subscribe to ticks
    let response = client.ticks(request).await?;
    println!("Symbol: {}, Quote: {}", response.tick.symbol, response.tick.quote);

    Ok(())
}
```

### Buying a Contract

```rust
use deriv_api::{DerivClient, Result};
use deriv_api_schema::*;

#[tokio::main]
async fn main() -> Result<()> {
    let client = DerivClient::new(
        "wss://ws.binaryws.com/websockets/v3",
        1234,
        "en",
        "https://your-app.com",
        None,
    )?;

    client.connect().await?;

    // Authorize
    let auth_request = AuthorizeRequest {
        authorize: "YOUR_API_TOKEN".to_string(),
        ..Default::default()
    };

    client.authorize(auth_request).await?;

    // Create a proposal request
    let proposal_request = ProposalRequest {
        proposal: 1,
        amount: 100.0,
        barrier: Some("+0.001".to_string()),
        contract_type: "CALL".to_string(),
        currency: "USD".to_string(),
        duration: 5,
        duration_unit: "t".to_string(),
        symbol: "R_50".to_string(),
        ..Default::default()
    };

    // Get proposal and buy contract
    let proposal = client.proposal(proposal_request).await?;

    Ok(())
}
```

## API Coverage

The client supports all endpoints from the official Deriv API, including but not limited to:

- Market data streaming (ticks, candles)
- Contract purchasing and management
- Account management
- Portfolio operations
- Payment operations

## Development

### Building

```bash
make build
```

### Running Tests

```bash
make test
```

### Generating Schema Types

```bash
make generate
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Disclaimer

This is an unofficial library and is not affiliated with Deriv.com.
