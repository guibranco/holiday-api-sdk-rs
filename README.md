# HolidayAPI Rust client

📆⚙️ [HolidayAPI](https://holidayapi.com/docs) client wrapper for Rust projects.

![GitHub last commit (branch)](https://img.shields.io/github/last-commit/guibranco/holiday-api-rust/main)
![Crates.io](https://img.shields.io/crates/d/holiday-api-rust)
[![wakatime](https://wakatime.com/badge/github/guibranco/holiday-api-rust.svg)](https://wakatime.com/badge/github/guibranco/holiday-api-rust)

[![Maintainability](https://api.codeclimate.com/v1/badges/392b044637f43eb881ac/maintainability)](https://codeclimate.com/github/guibranco/holiday-api-rust/maintainability)
[![Test Coverage](https://api.codeclimate.com/v1/badges/392b044637f43eb881ac/test_coverage)](https://codeclimate.com/github/guibranco/holiday-api-rust/test_coverage)
[![CodeFactor](https://www.codefactor.io/repository/github/guibranco/holiday-api-rust/badge)](https://www.codefactor.io/repository/github/guibranco/holiday-api-rust)

| Service | Status |
|--------|:------:|
| crates.io | [![Crates.io](https://img.shields.io/crates/v/holiday-api-rust.svg)](https://crates.io/crates/holiday-api-rust) |

Pure Rust bindings to the [Holiday API](https://holidayapi.com).

---

# Features

- Retrieve holidays by **country and year**
- List supported **countries**
- List supported **languages**
- Calculate the next **workday**
- Calculate the number of **workdays between dates**

---

# Supported platforms

`holiday_api_rust` works on all **Tier 1 Rust platforms**:

- macOS
- Linux
- Windows

---

# Minimum supported Rust version (MSRV)

This crate currently targets **Rust 1.70+** (Rust 2021 edition).

---

# Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
holiday_api_rust = "0.3"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
````

---

# Usage

Create a client with your HolidayAPI key and perform requests asynchronously.

```rust
use holiday_api_rust::HolidayAPIClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let client = HolidayAPIClient::new("YOUR_API_KEY".to_string());

    let holidays = client
        .search_holidays("2025", "US")
        .await?
        .unwrap_or_default();

    for holiday in holidays {
        println!(
            "Holiday: {} | Date: {} | Country: {}",
            holiday.name,
            holiday.date,
            holiday.country
        );
    }

    Ok(())
}
```

---

# Example: list supported countries

```rust
use holiday_api_rust::HolidayAPIClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let client = HolidayAPIClient::new("YOUR_API_KEY".to_string());

    let countries = client.search_countries().await?;

    for country in countries {
        println!("{} - {}", country.code, country.name);
    }

    Ok(())
}
```

---

# Example: get next workday

```rust
use holiday_api_rust::HolidayAPIClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let client = HolidayAPIClient::new("YOUR_API_KEY".to_string());

    let workday = client
        .workday("US", "2025-03-01", "5")
        .await?;

    println!("Next workday: {}", workday.date);

    Ok(())
}
```

---

# HolidayAPI

You can obtain an API key from:

[https://holidayapi.com](https://holidayapi.com)

Documentation:

[https://holidayapi.com/docs](https://holidayapi.com/docs)

---

# License

Licensed under the MIT license.

See the [LICENSE](https://github.com/guibranco/holiday-api-rust/blob/main/LICENSE) file for details.
