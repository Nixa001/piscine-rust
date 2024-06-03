#[derive(Debug, PartialEq, Eq)]
pub enum Security {
    Unknown,
    High,
    Medium,
    Low,
    BlockServer,
}

pub fn fetch_data(server: Result<String, String>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => server.expect("Panic without message"),
        Security::High => server.expect("ERROR: program stops"),
        Security::Medium => server.unwrap_or_else(|_| "WARNING: check the server".to_string()),
        Security::Low => server.map_err(|s| format!("Not found: {}", s)).unwrap_or_else(|_| "Not found: unknown".to_string()),
        Security::BlockServer => match server {
            Ok(url) => panic!("{}", url),
            Err(err) => err,
        },
    }
}

fn main() {
    println!("{}", fetch_data(Ok("server1.com".to_string()), Security::Medium));
    println!("{}", fetch_data(Err(String::new()), Security::Medium));
    println!("{}", fetch_data(Err("server2.com".to_string()), Security::Low));

    // Panics with no custom message
    // fetch_data(Err("ERROR CRITICAL".to_string()), Security::Unknown);

    // Panics with the message "ERROR: program stops"
    // fetch_data(Err(String::new()), Security::High);

    // Panics with the message "malicious_server.com"
    // fetch_data(Ok("malicious_server.com".to_string()), Security::BlockServer);
}

// And its output:

// $ cargo run
// server1.com
// WARNING: check the server
// Not found: server2.com
// $
