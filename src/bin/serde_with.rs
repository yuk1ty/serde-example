#![allow(unused)]

use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
struct Price {
    value: u64,
    currency: String,
}

#[derive(Deserialize, Debug)]
struct Item {
    #[serde(deserialize_with = "parse_with_unit")]
    price: Price,
}

fn parse_with_unit<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Price, D::Error> {
    let s = String::deserialize(deserializer)?;
    let mut split = s.splitn(2, ' ');
    let value = split.next().unwrap().parse().unwrap();
    let unit = split.next().unwrap().to_string();
    Ok(Price {
        value,
        currency: unit,
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json = json!({
        "price": "1000 JPY"
    });
    let item: Item = serde_json::from_value(json)?;
    println!("{:?}", item);
    Ok(())
}
