use chrono::serde::ts_seconds;
use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, Utc};
use rust_decimal::prelude::*;
use serde::{Deserialize, Deserializer};
use std::ops::Bound::{Excluded, Included};

use sqlx::postgres::{types::PgRange, PgPool};

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

async fn handle_csv(csv: String, pool: &PgPool) -> Option<usize> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(true)
        .from_reader(csv.as_bytes());
    let mut rows_added = 0;
    for row in rdr.records() {
        let r: Row = row.unwrap().clone().deserialize(None).unwrap();
        // println!("{} - {}", &r.timestamp, &r.price);
        let range: PgRange<NaiveDateTime> = PgRange {
            start: Included(r.timestamp.naive_utc()),
            end: Excluded(
                r.timestamp
                    .checked_add_signed(Duration::hours(1))
                    .unwrap()
                    .naive_utc(),
            ),
        };
        let res = sqlx::query(
            "INSERT INTO nordpool_price
                    (time, region, price)
                    VALUES($1, $2, $3)",
        )
        .bind(&range)
        .bind("ee")
        .bind(&r.price)
        .execute(pool)
        .await;

        if res.is_err() {
            println!("Exiting with error: {:?}", res);
            return None;
        }
        rows_added += 1;
    }
    if rows_added == 0 {
        return None
    }
    Some(rows_added)
}

#[derive(sqlx::FromRow, Debug)]
struct MaxDatetime {
    max: Option<NaiveDateTime>,
}

async fn fetch_csv(start_date: NaiveDateTime, duration: Duration, region: &str) -> String {
    let api_url = "https://dashboard.elering.ee/api/nps/price/csv";
    let end_date = start_date
        .checked_add_signed(duration)
        .and_then(|dt| dt.checked_sub_signed(Duration::seconds(1)))
        .unwrap();
    let body: String = ureq::get(api_url)
        .query(
            "start",
            &DateTime::<Utc>::from_utc(start_date, Utc).to_rfc3339(),
        )
        .query(
            "end",
            &DateTime::<Utc>::from_utc(end_date, Utc).to_rfc3339(),
        )
        .query("fields", region)
        .call()
        .unwrap()
        .into_string()
        .unwrap();
    body
}

#[tokio::main]
async fn main() {
    let pgsql_uri = match std::env::var("POSTGRESQL_URL") {
        Ok(val) => val,
        Err(_e) => {
            let v = "postgresql:/meters".to_string();
            println!("POSTGRESQL_URL not defined, using default: '{}", v);
            v
        },
    };
    let pool = PgPool::connect(&pgsql_uri).await.unwrap();

    let date_lookup = sqlx::query_as::<_, MaxDatetime>(
        "SELECT MAX(UPPER(time)) FROM nordpool_price WHERE region = $1",
    )
    .bind("ee".to_string())
    .fetch_one(&pool)
    .await
    .unwrap();

    let mut start_date = match date_lookup.max {
        Some(date) => date,
        None => NaiveDate::from_ymd(2018, 1, 1).and_hms(0, 0, 0),
    };

    let timespan = Duration::days(2);

    loop {

        let body: String = fetch_csv(start_date, timespan, "ee").await;

        println!("Fetching data starting from: {}", start_date);

        if let Some(_num_rows) = handle_csv(body, &pool).await {
            start_date = start_date.checked_add_signed(timespan).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(500));
            continue
        }
        break
    }
}
