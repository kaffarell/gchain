use std::io::stdin;
use transaction::Transaction;
use dotenv::dotenv;

mod crypto;
mod requests;


fn main() {
    // Load .env config
    dotenv().ok();
    // Check if new public/private key or use existing one
    println!("[1] Create new pub/priv key pair \n[2] Make Transaction \n[3] Show Info");
    let mut line = String::new();
    let _ = stdin().read_line(&mut line).unwrap();
    let choice_number: i32 = line.trim_end().parse().expect("Error parsing to i32");
    if choice_number == 1 {
        // Generate keys
        crypto::generate_keys();
    }else if choice_number == 2 {
        // Output public key
        println!("Your public key: \n{}", crypto::get_public_key());

        // Get Transaction
        println!("Enter recipient: ");
        let mut recipient_input = String::new();
        let _ = stdin().read_line(&mut recipient_input).unwrap();
        println!("Enter data (string): ");
        let mut data_input = String::new();
        let _ = stdin().read_line(&mut data_input).unwrap();

        // Slice public key out of wrapping BEGIN PUBLIC KEY
        let mut public_key_string = crypto::get_public_key()[27..crypto::get_public_key().len()-26].to_string();
        public_key_string = public_key_string.replace("\n", "").replace("\r", "");

        // Sign
        let mut tran: Transaction = Transaction{
            sender: public_key_string, 
            receiver: recipient_input, 
            data: data_input, 
            signature: vec![0]
        };
        crypto::sign(&mut tran);

        // Ask for confermation
        println!("-----------------------------------------------");
        println!("{}", tran);
        println!("-----------------------------------------------");
        println!("Is this transaction correct? \n[1] yes \n[2] no");
        let mut transaction_confermation_input = String::new();
        let _ = stdin().read_line(&mut transaction_confermation_input).unwrap();
        let confermation: i32 = transaction_confermation_input.trim_end().parse().expect("Error parsing to i32");
        if confermation == 2 {
            return; 
        }

        // Send to server
        requests::send_transaction(&tran);

    }else if choice_number == 3 {
        println!("-----------------------------------------------");
        println!("{}", crypto::get_public_key()[27..crypto::get_public_key().len()-26].to_string().replace("\n", ""));
        println!("-----------------------------------------------");
        println!("Public key:");
        println!("{}", crypto::get_public_key());
        println!("Private key:");
        println!("{}", crypto::get_private_key());
        println!("Blockchain status:");
        println!("{}", requests::uptime_check());
    }else{
        println!("Input error");
    }
}
