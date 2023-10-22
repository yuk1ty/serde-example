use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    price: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Items {
    items: Vec<Item>,
    #[serde(flatten)]
    pagination: Pagination,
}

#[derive(Serialize, Deserialize, Debug)]
struct Pagination {
    offset: u64,
    limit: u64,
    total: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let items = Items {
        items: vec![Item { price: 100 }, Item { price: 200 }],
        pagination: Pagination {
            offset: 0,
            limit: 10,
            total: 100,
        },
    };

    println!("{}", serde_json::to_string_pretty(&items)?);

    Ok(())
}
