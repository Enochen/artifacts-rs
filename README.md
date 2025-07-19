# artifacts-rs

A Rust client for [Artifacts](https://artifactsmmo.com/), an API-based sandbox MMO.

Powered by [openapi-generator](https://openapi-generator.tech/).

## Example Usage

```rust
use artifacts::{
    apis::{configuration::Configuration, my_account_api},
    models::BankResponseSchema,
};

#[tokio::main]
async fn main() {
    let mut config = Configuration::new();
    config.bearer_access_token = Some("YOUR_TOKEN".to_owned());

    match my_account_api::get_bank_details(&config).await {
        Ok(BankResponseSchema { data }) => println!("{:?}", data),
        Err(err) => panic!("{}", err),
    }
}
```
