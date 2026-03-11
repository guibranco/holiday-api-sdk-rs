use dotenv::dotenv;
use holiday_api_rust::HolidayAPIClient;
use prettytable::{format, row, Table};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("HOLIDAYAPI_APIKEY").unwrap();
    let client = HolidayAPIClient::new(api_key);

    let year = env::args().nth(1).expect("year");
    let country = env::args().nth(2).expect("country");

    match client.search_holidays(&year, &country).await {
        Err(e) => eprintln!("{:?}", e),
        Ok(holidays) => match holidays {
            None => println!("No holidays!"),
            Some(h) => print_holidays(h),
        },
    }
}

fn print_holidays(holidays: Vec<holiday_api_rust::Holiday>) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(row!["Name", "Date", "Country"]);
    for holiday in holidays {
        table.add_row(row![holiday.name, holiday.date, holiday.country]);
    }
    table.printstd();
}
