use transaction::Transaction;


pub fn send_transaction(transaction: &Transaction) {
    let string = serde_json::to_string(transaction).unwrap();
    let client = reqwest::blocking::Client::new();
    let res = client.post("http://localhost:8000/add").body(string).send().unwrap();
    println!("{:?}", res);
}