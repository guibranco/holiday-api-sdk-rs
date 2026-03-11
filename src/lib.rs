use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use url::Url;

#[derive(Deserialize, Debug)]
pub struct Holidays {
    pub status: u32,
    pub error: Option<String>,
    pub warning: Option<String>,
    pub requests: Requests,
    pub holidays: Option<Vec<Holiday>>,
}

#[derive(Deserialize, Debug)]
pub struct Requests {
    pub used: u32,
    pub available: u32,
    pub resets: String,
}

#[derive(Deserialize, Debug)]
pub struct Holiday {
    pub name: String,
    pub date: String,
    pub observed: String,
    pub public: bool,
    pub country: String,
    pub uuid: String,
    pub weekday: Weekday,
}

#[derive(Deserialize, Debug)]
pub struct Weekday {
    pub date: WeekDate,
    pub observed: WeekDate,
}

#[derive(Deserialize, Debug)]
pub struct WeekDate {
    pub name: String,
    pub numeric: String,
}

#[derive(Deserialize, Debug)]
pub struct Countries {
    pub status: u32,
    pub error: Option<String>,
    pub warning: Option<String>,
    pub requests: Requests,
    pub countries: Vec<Country>,
}

#[derive(Deserialize, Debug)]
pub struct Country {
    pub code: String,
    pub name: String,
    pub languages: Vec<String>,
    pub codes: Codes,
    pub flag: String,
    pub subdivisions: Vec<Subdivision>,
}

#[derive(Deserialize, Debug)]
pub struct Codes {
    #[serde(rename = "alpha-2")]
    pub alpha_2: String,

    #[serde(rename = "alpha-3")]
    pub alpha_3: String,

    pub numeric: String,
}

#[derive(Deserialize, Debug)]
pub struct Subdivision {
    pub code: String,
    pub name: String,
    pub languages: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Languages {
    pub status: u32,
    pub error: Option<String>,
    pub warning: Option<String>,
    pub requests: Requests,
    pub languages: Vec<Language>,
}

#[derive(Deserialize, Debug)]
pub struct Language {
    pub code: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct WorkdayResponse {
    pub status: u32,
    pub error: Option<String>,
    pub warning: Option<String>,
    pub requests: Requests,
    pub workday: Workday,
}

#[derive(Deserialize, Debug)]
pub struct Workday {
    pub date: String,
    pub weekday: WeekDate,
}

#[derive(Deserialize, Debug)]
pub struct Workdays {
    pub status: u32,
    pub error: Option<String>,
    pub warning: Option<String>,
    pub requests: Requests,
    pub workdays: u32,
}

pub struct HolidayAPIClient {
    api_key: String,
    api_base: String,
    client: Client,
}

impl HolidayAPIClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            api_base: "https://holidayapi.com/v1/".to_string(),
            client: Client::new(),
        }
    }

    fn build_url(&self, path: &str) -> Result<Url, url::ParseError> {
        let mut url = Url::parse(&self.api_base)?.join(path)?;
        url.query_pairs_mut().append_pair("key", &self.api_key);
        Ok(url)
    }

    pub async fn search_holidays(
        &self,
        year: &str,
        country: &str,
    ) -> Result<Option<Vec<Holiday>>, Box<dyn Error>> {
        let mut url = self.build_url("holidays")?;
        url.query_pairs_mut()
            .append_pair("year", year)
            .append_pair("country", country);

        let response: Holidays = self.client.get(url).send().await?.json().await?;

        Ok(response.holidays)
    }

    pub async fn search_countries(&self) -> Result<Vec<Country>, Box<dyn Error>> {
        let url = self.build_url("countries")?;

        let response: Countries = self.client.get(url).send().await?.json().await?;

        Ok(response.countries)
    }

    pub async fn search_languages(&self) -> Result<Vec<Language>, Box<dyn Error>> {
        let url = self.build_url("languages")?;

        let response: Languages = self.client.get(url).send().await?.json().await?;

        Ok(response.languages)
    }

    pub async fn workday(
        &self,
        country: &str,
        start: &str,
        days: &str,
    ) -> Result<Workday, Box<dyn Error>> {
        let mut url = self.build_url("workday")?;

        url.query_pairs_mut()
            .append_pair("country", country)
            .append_pair("start", start)
            .append_pair("days", days);

        let response: WorkdayResponse = self.client.get(url).send().await?.json().await?;

        Ok(response.workday)
    }

    pub async fn workdays(
        &self,
        country: &str,
        start: &str,
        end: &str,
    ) -> Result<u32, Box<dyn Error>> {
        let mut url = self.build_url("workdays")?;

        url.query_pairs_mut()
            .append_pair("country", country)
            .append_pair("start", start)
            .append_pair("end", end);

        let response: Workdays = self.client.get(url).send().await?.json().await?;

        Ok(response.workdays)
    }
}
