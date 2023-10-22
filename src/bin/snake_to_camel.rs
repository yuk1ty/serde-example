use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RenameAllFields {
    rename_this: i32,
    rename_that: bool,
}

#[derive(Serialize, Deserialize)]
struct RenameOnlySpecificField {
    #[serde(rename = "wow")]
    renamed_field: i32,
    do_not_rename: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let all_fields = RenameAllFields {
        rename_this: 1,
        rename_that: true,
    };
    println!("{}", serde_json::to_string_pretty(&all_fields)?);

    let specific_field = RenameOnlySpecificField {
        renamed_field: 42,
        do_not_rename: false,
    };
    println!("{}", serde_json::to_string_pretty(&specific_field)?);

    Ok(())
}
