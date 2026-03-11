# HolidayAPI Rust client

📆⚙️ [HolidayAPI](https://holidayapi.com/docs) client wrapper for Rust projects.

![GitHub last commit (branch)](https://img.shields.io/github/last-commit/guibranco/holiday-api-sdk-rs/main)
[![wakatime](https://wakatime.com/badge/github/guibranco/holiday-api-sdk-rs.svg)](https://wakatime.com/badge/github/guibranco/holiday-api-sdk-rs)

[![Maintainability](https://api.codeclimate.com/v1/badges/392b044637f43eb881ac/maintainability)](https://codeclimate.com/github/guibranco/holiday-api-sdk-rs/maintainability)
[![Test Coverage](https://api.codeclimate.com/v1/badges/392b044637f43eb881ac/test_coverage)](https://codeclimate.com/github/guibranco/holiday-api-sdk-rs/test_coverage)
[![CodeFactor](https://www.codefactor.io/repository/github/guibranco/holiday-api-sdk-rs/badge)](https://www.codefactor.io/repository/github/guibranco/holiday-api-sdk-rs)

| Service   | Status |
| --------- | :----: |
| crates.io | [![Crates.io](https://img.shields.io/crates/v/holiday_api.svg)](https://crates.io/crates/holiday_api) |

Pure Rust bindings to the [Holiday API](https://holidayapi.com).

## Dependencies and support

`holiday_api` is intended to work on all tier 1 supported Rust systems:

- macOS
- Linux
- Windows

## Minimum Compiler Version

`holiday_api` requires `rustc` **1.75** or higher (edition 2021, async/await).

## Getting Started

Add the following to your `Cargo.toml`:

```toml
[dependencies]
holiday_api = "1.0.0"
tokio = { version = "1", features = ["full"] }
```

Then in your `main.rs`:

```rust
use holiday_api::HolidayAPIClient;

#[tokio::main]
async fn main() {
    let client = HolidayAPIClient::new("YOUR_HOLIDAY_API_KEY".to_string());

    match client.search_holidays("2019", "BR").await {
        Err(e) => eprintln!("{:?}", e),
        Ok(Some(holidays)) => {
            for holiday in holidays {
                println!(
                    "Holiday: {} | Date: {} | Country: {}",
                    holiday.name, holiday.date, holiday.country
                );
            }
        }
        Ok(None) => println!("No holidays found."),
    }
}
```

## Available methods

All methods are `async` and must be `.await`ed.

| Method | Description |
| ------ | ----------- |
| `search_holidays(year, country)` | Returns holidays for a given year and country code |
| `search_countries()` | Returns a list of all supported countries |
| `search_languages()` | Returns a list of all supported languages |
| `workday(country, start, days)` | Returns the workday date after N working days from a start date |
| `workdays(country, start, end)` | Returns the number of working days between two dates |

## Environment variable

The API key can be loaded from a `.env` file using [dotenv](https://crates.io/crates/dotenv):

```env
HOLIDAYAPI_APIKEY=your_api_key_here
```

```rust
use dotenv::dotenv;
use std::env;

dotenv().ok();
let api_key = env::var("HOLIDAYAPI_APIKEY").unwrap();
let client = HolidayAPIClient::new(api_key);
```

## License

Licensed under the MIT license ([LICENSE](https://github.com/guibranco/holiday-api-sdk-rs/blob/main/LICENSE) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT)).
