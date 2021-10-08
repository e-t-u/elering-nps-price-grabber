use chrono::serde::ts_seconds;
use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, Utc};
use rust_decimal::prelude::*;
use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
struct Row {
    #[serde(with = "ts_seconds")]
    timestamp: DateTime<Utc>,
    _ts: String,
    #[serde(deserialize_with = "deserialize_decimal")]
    price: Decimal,
}

pub fn deserialize_decimal<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?.replace(",", ".");
    Decimal::from_str(&s).map_err(serde::de::Error::custom)
}

pub fn handle_csv(csv: String) {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(true)
        .from_reader(csv.as_bytes());
    for row in rdr.records() {
        let r: Row = row.unwrap().clone().deserialize(None).unwrap();
        println!("{} - {}", &r.timestamp, &r.price);
    }
}

pub fn main() {
    let start_date: NaiveDateTime = NaiveDate::from_ymd(2018, 1, 1).and_hms(0, 0, 0);

    let end_date = start_date
        .checked_add_signed(Duration::days(1))
        .and_then(|dt| dt.checked_sub_signed(Duration::seconds(1)))
        .unwrap();

    let url = "https://dashboard.elering.ee/api/nps/price/csv";

    let body: String = ureq::get(url)
        .query(
            "start",
            &DateTime::<Utc>::from_utc(start_date, Utc).to_rfc3339(),
        )
        .query(
            "end",
            &DateTime::<Utc>::from_utc(end_date, Utc).to_rfc3339(),
        )
        .query("fields", "ee")
        .call()
        .unwrap()
        .into_string()
        .unwrap();

    handle_csv(body);
}
