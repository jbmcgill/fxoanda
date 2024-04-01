//! This is an unofficial [Oanda](https://wwww.oanda.com/) API client. This client is still
//! an experimental work in progress however it is reasonably functional.
//!
//! The client is generated from the Oanda V20 API definitions. The current state of the client
//! API is low-level but usable however I would like to see a more ergonomic layer developed on
//! top.
//!
//! # Installation
//!
//! ```bash
//! $ cargo add fxoanda
//! ```
//!
//! # Example usage
//!
//! ```
//! use std::env;
//! use fxoanda::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let api_key = env::var("OANDA_KEY").expect("expected OANDA_KEY environment variable to be set");
//!     let api_host = env::var("OANDA_HOST").expect("expected OANDA_HOST environment variable to be set");
//!
//!     let client = fxoanda::Client {
//!         host: String::from(api_host),
//!         reqwest: reqwest::Client::new(),
//!         authentication: String::from(api_key),
//!     };
//!     match fxoanda::GetInstrumentCandlesRequest::new()
//!         .with_instrument("EUR_USD".to_string())
//!         .with_granularity(CandlestickGranularity::H4)
//!         .remote(&client).await
//!     {
//!         Ok(x) => println!("OK: {:#?}", x),
//!         Err(e) => eprintln!("ERR: {:#?}", e),
//!     };
//! }
//! ```
//!
//! # Warning
//!
//! Forex markets are extremely risky. Automated trading is also extremely risky.
//! This project is extremely risky. Market conditions, news events, or software bugs
//! can wipe out your account in an instant.
//!
//! # Disclaimer
//!
//! Use this project at your own risk. The maintainers of this project make no
//! claims as to this product being fit for purpose. In fact, the maintainers of this
//! project are telling you that you shouldn't use this project.
//!

#![crate_type = "lib"]
#[allow(unused_imports)]
#[macro_use]
extern crate serde_derive;
//#[macro_use]
extern crate serde;
//#[macro_use]
extern crate chrono;
extern crate fxoanda_definitions;
extern crate fxoanda_serdes;
extern crate serde_json;
extern crate time;

pub mod account;
pub mod client;
pub mod instrument;
pub mod pricing;
pub use self::account::*;
pub use self::client::*;
pub use self::instrument::*;
pub use self::pricing::*;
pub use fxoanda_definitions::*;
pub use fxoanda_serdes::*;
