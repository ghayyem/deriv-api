
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: ./deriv-api-docs/config/v3/get_limits/receive.json

// Use direct crate names for imports within generated files
use serde::{Deserialize, Serialize}; 




// Import shared types from the *same* crate
use crate::fiat_to_crypto::FiatToCrypto; 
use crate::crypto_to_crypto::CryptoToCrypto; 
use crate::crypto_to_fiat::CryptoToFiat; 

// It's a struct
/// Lifetime transfer limits. Only present when applicable to the current accout.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LifetimeTransfers {
    /// Lifetime transfer limit for crypto to crypto currencies.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub crypto_to_crypto: Option<CryptoToCrypto>,
    /// Lifetime transfer limit for crypto to fiat currencies.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub crypto_to_fiat: Option<CryptoToFiat>,
    /// Lifetime transfer limit for fiat to crypto currencies.\n
    // Correct serde attribute construction - Use helper
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub fiat_to_crypto: Option<FiatToCrypto>,
}

