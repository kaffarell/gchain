use transaction::Transaction;
use std::time::Instant;

use dotenv;

pub fn send_transaction(transaction: &Transaction) {
    // Search for variable in the dotenv file
    let url: String = dotenv::var("SERVER_URL").unwrap(); 

    let string = serde_json::to_string(transaction).unwrap();
    let client = reqwest::blocking::Client::new();
    let res = client.post(url + "add").body(string).send().unwrap();
    println!("{:?}", res);
}

pub fn uptime_check() -> String {
    // Get server url from env
    let url: String = dotenv::var("SERVER_URL").unwrap(); 

    // Make request to the url and measure the execution time
    let start_time = Instant::now();

    let client = reqwest::blocking::Client::new();

    // The / route (url) returns a status (200 or nothing)
    let res_result  = client.get(url).send();
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