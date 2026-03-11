use dotenv::dotenv;
use holiday_api_rust::HolidayAPIClient;
use prettytable::{Table, format};
use std::env;

#[macro_use]
extern crate prettytable;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("HOLIDAYAPI_APIKEY").unwrap();

    let client = HolidayAPIClient::new(api_key);

    match client.search_languages().await {
        Err(e) => eprintln!("{:?}", e),
        Ok(languages) => {
            let mut table = Table::new();
            table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            table.set_titles(row!["Code", "Name"]);

            for language in languages {
                table.add_row(row![language.code, language.name]);
            }

            table.printstd();
        }
    }
}
