use chrono::{DateTime, Utc, Duration};
use rand::Rng;
use rayon::prelude::*;
use serde::Serialize;
use serde_json::json;
use std::fs::OpenOptions;
use std::io::Write;
use uuid::Uuid;

#[derive(Serialize)]
struct LogEntry {
    #[serde(rename = "@t")]
    timestamp: String,
    #[serde(rename = "@mt")]
    message_template: String,
    #[serde(rename = "UserId")]
    user_id: String,
    #[serde(rename = "Action")]
    action: String,
    #[serde(rename = "Counter")]
    counter: u64,
}

fn generate_log_entry(counter: u64, timestamp: DateTime<Utc>) -> LogEntry {
    let actions = ["Login", "Logout", "Update", "Delete"];
    let user_id = Uuid::new_v4().to_string();
    let action = actions[rand::thread_rng().gen_range(0..actions.len())];

    LogEntry {
        timestamp: timestamp.to_rfc3339(),
        message_template: format!(
            "User {{UserId}} performed action {{Action}}, counter is {{Counter}}"
        ),
        user_id,
        action: action.to_string(),
        counter,
    }
}

fn write_logs_to_file(file_name: &str, logs: &[LogEntry]) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_name)?;

    for log in logs {
        let log_json = json!(log).to_string();
        writeln!(file, "{}", log_json)?;
    }

    Ok(())
}

fn generate_logs_for_environment(env: &str, log_count: u64, start_time: DateTime<Utc>, interval_per_log: Duration) {
    let file_name = format!("{}_logs.json", env);

    let logs: Vec<LogEntry> = (0..log_count)
        .into_par_iter()  // Parallel iterator
        .map(|counter| {
            let timestamp = start_time + interval_per_log * (counter as i32);
            generate_log_entry(counter, timestamp)
        })
        .collect();

    write_logs_to_file(&file_name, &logs).expect("Unable to write logs to file");

    println!("Logs generated for environment: {}", env);
}

fn main() {
    let environments = ["Development", "Testing", "Acceptance", "Production"];
    let log_count_per_env = 1_000_000; // Adjust the number of logs per environment as needed

    // Start time set to two years ago
    let start_time = Utc::now() - Duration::days(365 * 2);
    let end_time = Utc::now();
    let total_duration = end_time - start_time;

    // Calculate the interval per log entry for each environment separately
    let interval_per_log = total_duration / (log_count_per_env as i32);

    environments
        .par_iter()  // Parallel iterator for environments
        .for_each(|env| {
            generate_logs_for_environment(env, log_count_per_env, start_time, interval_per_log);
        });
}
