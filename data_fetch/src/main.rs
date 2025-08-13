use serde::Deserialize;
use ureq;
use serde_json;
use chrono::{Utc, TimeZone};
use std::io::Write;
use std::{thread, time::Duration};

//structs
#[derive(Debug, Deserialize)]
pub struct Bitcoin {
    pub price: f64,
    #[serde(rename = "last_updated_at")]
    pub timestamp: i64,
}

#[derive(Debug, Deserialize)]
pub struct Ethereum {
    pub price: f64,
    #[serde(rename = "last_updated_at")]
    pub timestamp: i64,
}

#[derive(Debug, Deserialize)]
pub struct SP500 {
    pub price: f64,
    pub timestamp: i64,
}

//pricing trait
pub trait Pricing {
    fn fetch_price(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>>;
}

impl Pricing for Bitcoin {
    fn fetch_price(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let response = ureq::get("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd&include_last_updated_at=true")
            .call()?
            .into_string()?;
        
        let data: serde_json::Value = serde_json::from_str(&response)?;
        let price = data["bitcoin"]["usd"].as_f64().unwrap_or(0.0);
        let timestamp = data["bitcoin"]["last_updated_at"].as_i64().unwrap_or(0);
        
        self.price = price;
        self.timestamp = timestamp;
        Ok(())
    }

    fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let timestamp_dt = Utc.timestamp_opt(self.timestamp, 0).unwrap();
        let formatted_timestamp = timestamp_dt.format("%Y-%m-%d %H:%M:%S").to_string();
        let filename = "bitcoin_prices.txt";
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(filename)?;
        writeln!(file, "{},{}", formatted_timestamp, self.price)?; // Use writeln! for line breaks
        Ok(())
    }
}

impl Pricing for Ethereum {
    fn fetch_price(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let response = ureq::get("https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd&include_last_updated_at=true")
            .call()?
            .into_string()?;

        let data: serde_json::Value = serde_json::from_str(&response)?;
        let price = data["ethereum"]["usd"].as_f64().unwrap_or(0.0);
        let timestamp = data["ethereum"]["last_updated_at"].as_i64().unwrap_or(0);

        self.price = price;
        self.timestamp = timestamp;
        Ok(())
    }

    fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let timestamp_dt = Utc.timestamp_opt(self.timestamp, 0).unwrap();
        let formatted_timestamp = timestamp_dt.format("%Y-%m-%d %H:%M:%S").to_string();
        let filename = "ethereum_prices.txt";
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(filename)?;
        writeln!(file, "{},{}", formatted_timestamp, self.price)?;
        Ok(())
    }
}

impl Pricing for SP500 {
    fn fetch_price(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let response = ureq::get("https://query2.finance.yahoo.com/v8/finance/chart/%5EGSPC")
            .call()?
            .into_string()?;

        let data: serde_json::Value = serde_json::from_str(&response)?;
        let price = data["sp500"]["usd"].as_f64().unwrap_or(0.0);
        let timestamp = data["sp500"]["last_updated_at"].as_i64().unwrap_or(0);

        self.price = price;
        self.timestamp = timestamp;
        Ok(())
    }

    fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let timestamp_dt = Utc.timestamp_opt(self.timestamp, 0).unwrap();
        let formatted_timestamp = timestamp_dt.format("%Y-%m-%d %H:%M:%S").to_string();
        let filename = "sp500_prices.txt";
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(filename)?;
        writeln!(file, "{},{}", formatted_timestamp, self.price)?;
        Ok(())
    }
}

//main function
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut bitcoin = Bitcoin { price: 0.0, timestamp: 0 };
    let mut ethereum = Ethereum { price: 0.0, timestamp: 0 };
    let mut sp500 = SP500 { price: 0.0, timestamp: 0 };

    let mut assets: Vec<&mut dyn Pricing> = vec![&mut bitcoin, &mut ethereum, &mut sp500];

    loop {
        for asset in &mut assets {
            match asset.fetch_price() {
                Ok(_) => println!("Fetched price for asset."),
                Err(e) => eprintln!("Error fetching price: {}", e),
            }
            match asset.save_to_file() {
                Ok(_) => println!("Saved price to file."),
                Err(e) => eprintln!("Error saving price: {}", e),
            }
        }
        thread::sleep(Duration::from_secs(10));
    }
}