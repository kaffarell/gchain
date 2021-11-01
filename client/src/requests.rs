use transaction::Transaction;
use reqwest::StatusCode;
use std::time::{Duration, Instant};


pub fn send_transaction(transaction: &Transaction) {
    let string = serde_json::to_string(transaction).unwrap();
    let client = reqwest::blocking::Client::new();
    let res = client.post("http://localhost:8000/add").body(string).send().unwrap();
    println!("{:?}", res);
}

pub fn uptime_check() -> String {
    // Make request to the url and measure the execution time
    let start_time = Instant::now();

    let client = reqwest::blocking::Client::new();
    let res_result  = client.get("http://localhost:8000/").send();
    match res_result {
        Ok(res) => {
            let duration = start_time.elapsed();
            if res.status().is_success() {
                return format!("UP ({:?})", duration);
            }else {
                return String::from("DOWN");
            }

        }
        Err(_) => {
            return String::from("DOWN");
        }
    }

}