use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct UserId(String);

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: UserId,
    name: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user = User {
        id: UserId("aaa".into()),
        name: "paild".into(),
    };
    println!("{}", serde_json::to_string_pretty(&user)?);

    let json = json!({
        "id": "aaa",
        "name": "paild"
    });
    let user = serde_json::from_value::<User>(json)?;
    println!("{:?}", user);

    Ok(())
}
