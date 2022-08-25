pub mod command;

use std::io;

pub const APP_NAME: &str = "avalanche-telemetry-cloudwatch";

#[tokio::main]
async fn main() -> io::Result<()> {
    let matches = command::new().get_matches();

    let log_level = matches.value_of("LOG_LEVEL").unwrap_or("info").to_string();

    let initial_wait_seconds = matches.value_of("INITIAL_WAIT_SECONDS").unwrap_or("10");
    let initial_wait_seconds = initial_wait_seconds.parse::<u32>().unwrap();

    let fetch_interval_seconds = matches.value_of("FETCH_INTERVAL_SECONDS").unwrap_or("60");
    let fetch_interval_seconds = fetch_interval_seconds.parse::<u32>().unwrap();

    let opts = command::Flags {
        log_level,
        initial_wait_seconds,
        fetch_interval_seconds,

        rules_file_path: matches
            .value_of("RULES_FILE_PATH")
            .unwrap_or("/data/avalanche-telemetry-cloudwatch.rules.yaml")
            .to_string(),
        namespace: matches
            .value_of("NAMESPACE")
            .unwrap_or("avalanche-telemetry-cloudwatch")
            .to_string(),

        rpc_endpoint: matches
            .value_of("RPC_ENDPOINT")
            .unwrap_or("http://localhost:9650")
            .to_string(),
    };
    command::execute(opts).await
}
