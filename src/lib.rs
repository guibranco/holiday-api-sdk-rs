use log::debug;
use reqwest::Client;
use serde::Deserialize;
use std::io;
use url::Url;

fn to_io_error<E>(err: E) -> io::Error
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    io::Error::other(err)
}

// ── Response structs ──────────────────────────────────────────────────────────

#[derive(Deserialize, Debug)]
pub struct Requests {
    pub used: u32,
    pub available: u32,
    pub resets: String,
}

#[derive(Deserialize, Debug)]
pub struct WeekDate {
    pub name: String,
    pub numeric: String,
}

#[derive(Deserialize, Debug)]
pub struct Weekday {
    pub date: WeekDate,
    pub observed: WeekDate,
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
struct Holidays {
    pub error: Option<String>,
    pub holidays: Option<Vec<Holiday>>,
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
pub struct Country {
    pub code: String,
    pub name: String,
    pub languages: Vec<String>,
    pub codes: Codes,
    pub flag: String,
    pub subdivisions: Vec<Subdivision>,
}

#[derive(Deserialize, Debug)]
struct Countries {
    pub countries: Vec<Country>,
}

#[derive(Deserialize, Debug)]
pub struct Language {
    pub code: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
struct Languages {
    pub languages: Vec<Language>,
}

#[derive(Deserialize, Debug)]
pub struct Workday {
    pub date: String,
    pub weekday: WeekDate,
}

#[derive(Deserialize, Debug)]
struct WorkdayResponse {
    pub workday: Workday,
}

#[derive(Deserialize, Debug)]
struct WorkdaysResponse {
    pub workdays: u32,
}

// ── URL builder ───────────────────────────────────────────────────────────────

struct UriMaker {
    api_key: String,
    api_base: String,
}

impl UriMaker {
    pub fn new(api_key: String, api_base: String) -> Self {
        UriMaker { api_key, api_base }
    }

    fn build_url(&self, path: &str) -> Url {
        let mut url = Url::parse(&self.api_base)
            .unwrap()
            .join(path)
            .unwrap();
        url.query_pairs_mut().append_pair("key", &self.api_key);
        url
    }

    pub fn holidays_by_country_and_year(&self, year: &str, country: &str) -> Url {
        let mut url = self.build_url("holidays");
        url.query_pairs_mut()
            .append_pair("year", year)
            .append_pair("country", country);
        url
    }

    pub fn countries(&self) -> Url {
        self.build_url("countries")
    }

    pub fn languages(&self) -> Url {
        self.build_url("languages")
    }

    pub fn workday(&self, country: &str, start: &str, days: &str) -> Url {
        let mut url = self.build_url("workday");
        url.query_pairs_mut()
            .append_pair("country", country)
            .append_pair("start", start)
            .append_pair("days", days);
        url
    }

    pub fn workdays(&self, country: &str, start: &str, end: &str) -> Url {
        let mut url = self.build_url("workdays");
        url.query_pairs_mut()
            .append_pair("country", country)
            .append_pair("start", start)
            .append_pair("end", end);
        url
    }
}

// ── Client ────────────────────────────────────────────────────────────────────

pub struct HolidayAPIClient {
    uri_maker: UriMaker,
    http: Client,
}

impl HolidayAPIClient {
    pub fn new(api_key: String) -> Self {
        let http = Client::builder()
            .build()
            .expect("Failed to build HTTP client");
        let uri_maker = UriMaker::new(api_key, "https://holidayapi.com/v1/".to_owned());
        HolidayAPIClient { uri_maker, http }
    }

    async fn get_json(&self, url: Url) -> Result<serde_json::Value, io::Error> {
        debug!("GET {}", url);
        let response = self
            .http
            .get(url)
            .send()
            .await
            .map_err(to_io_error)?;
        debug!("Response: {}", response.status());
        response.json::<serde_json::Value>().await.map_err(to_io_error)
    }

    pub async fn search_holidays(
        &self,
        year: &str,
        country: &str,
    ) -> Result<Option<Vec<Holiday>>, io::Error> {
        let url = self.uri_maker.holidays_by_country_and_year(year, country);
        let value = self.get_json(url).await?;
        let wrapper: Holidays = serde_json::from_value(value).map_err(to_io_error)?;
        match &wrapper.error {
            None => debug!("Success"),
            Some(e) => debug!("Error: {}", e),
        }
        Ok(wrapper.holidays)
    }

    pub async fn search_countries(&self) -> Result<Vec<Country>, io::Error> {
        let url = self.uri_maker.countries();
        let value = self.get_json(url).await?;
        let wrapper: Countries = serde_json::from_value(value).map_err(to_io_error)?;
        Ok(wrapper.countries)
    }

    pub async fn search_languages(&self) -> Result<Vec<Language>, io::Error> {
        let url = self.uri_maker.languages();
        let value = self.get_json(url).await?;
        let wrapper: Languages = serde_json::from_value(value).map_err(to_io_error)?;
        Ok(wrapper.languages)
    }

    pub async fn workday(
        &self,
        country: &str,
        start: &str,
        days: &str,
    ) -> Result<Workday, io::Error> {
        let url = self.uri_maker.workday(country, start, days);
        let value = self.get_json(url).await?;
        let wrapper: WorkdayResponse = serde_json::from_value(value).map_err(to_io_error)?;
        Ok(wrapper.workday)
    }

    pub async fn workdays(
        &self,
        country: &str,
        start: &str,
        end: &str,
    ) -> Result<u32, io::Error> {
        let url = self.uri_maker.workdays(country, start, end);
        let value = self.get_json(url).await?;
        let wrapper: WorkdaysResponse = serde_json::from_value(value).map_err(to_io_error)?;
        Ok(wrapper.workdays)
    }
}
