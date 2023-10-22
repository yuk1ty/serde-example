use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    executed_by: String,
    status: TaskStatus,
}

// このコードは上手にデシリアライズできませんが、下記のようにすることで正しくデシリアライズすることができるようになります。
//
// #[derive(Serialize, Deserialize, Debug)]
// #[serde(untagged)]
// enum TaskStatus {
//     Failed { finished_at: String, reason: String },
//     Succeeded { finished_at: String },
// }

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum TaskStatus {
    Succeeded { finished_at: String },
    Failed { finished_at: String, reason: String },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = json!([
      {
        "executed_by": "admin",
        "status": {
          "finished_at": "2020-01-01 12:00:00"
        }
      },
      {
        "executed_by": "admin",
        "status": {
          "finished_at": "2020-01-01 12:00:00",
          "reason": "Task failed because of timeout"
        }
      }
    ]);
    let tasks: Vec<Task> = serde_json::from_value(source)?;
    println!("{:#?}", tasks);

    Ok(())
}
