use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Task {
    executed_by: String,
    status: TaskStatus,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum TaskStatus {
    Succeeded { finished_at: String },
    Failed { failed_at: String, reason: String },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tasks = vec![
        Task {
            executed_by: "admin".into(),
            status: TaskStatus::Succeeded {
                finished_at: "2020-01-01 12:00:00".into(),
            },
        },
        Task {
            executed_by: "admin".into(),
            status: TaskStatus::Failed {
                failed_at: "2020-01-01 12:00:00".into(),
                reason: "Task failed because of timeout".into(),
            },
        },
    ];
    println!("{}", serde_json::to_string_pretty(&tasks)?);
    Ok(())
}
