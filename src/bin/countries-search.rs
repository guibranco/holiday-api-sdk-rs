use dotenv::dotenv;
use holiday_api::HolidayAPIClient;
use prettytable::{format, row, Table};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("HOLIDAYAPI_APIKEY").unwrap();
    let client = HolidayAPIClient::new(api_key);

    match client.search_countries().await {
        Err(e) => eprintln!("{:?}", e),
        Ok(countries) => {
            let mut table = Table::new();
            table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            table.set_titles(row!["Code", "Name", "Flag"]);
            for country in countries {
                table.add_row(row![country.code, country.name, country.flag]);
            }
            table.printstd();
        }
    }
}
