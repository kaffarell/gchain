use md5;
use openssl::sign::{Verifier};
use openssl::rsa::Rsa;
use openssl::pkey::PKey;
use openssl::hash::MessageDigest;
use crate::payload::data::Transaction;

pub fn hash_md5<T: AsRef<[u8]>>(t: T) -> String {
    let digest = md5::compute(t);
    return format!("{:x}", digest);
}

/// Verifies if the signature of the transaction is valid.
/// 
/// Extracts the public key from the transaction, converts it into pem format (adding BEGIN PUBLIC KEY and END PUBLIC KEY)
/// then checks if the signature is correct.
pub fn verify_transaction(transaction: &Transaction) -> bool {
    println!("Verifying transaction");
    let mut pub_key_string: String = "-----BEGIN PUBLIC KEY-----\n".to_string();
    pub_key_string.push_str(&transaction.sender[..]);
    pub_key_string.push_str("\n-----END PUBLIC KEY-----");

    let pub_key = Rsa::public_key_from_pem(&pub_key_string.as_bytes()).expect("Error pem to public key");
    let keypair = PKey::from_rsa(pub_key).unwrap();

    let data = format!("{}", transaction);
    let signature = &transaction.signature;


    let mut verifier = Verifier::new(MessageDigest::sha256(), &keypair).unwrap();
    verifier.update(data.as_bytes()).unwrap();
    return verifier.verify(&signature[..]).unwrap();
}


#[cfg(test)]
mod tests {
    use openssl::sign::{Signer};
    use super::*;

    #[test]
    fn test_md5_hash(){
        let test_string = "test".as_bytes();
        let expected_result: String = String::from("098f6bcd4621d373cade4e832627b4f6");
        let result: String = hash_md5(test_string);
        assert_eq!(result, expected_result);
    }

    /// Create test transaction, sign it with a pre-defined private key and verify
    /// the signature with the public key
    #[test]
    fn test_verify_transaction() {
        // Sample private and public keys for testing
        /*
        -----BEGIN RSA PRIVATE KEY-----
        MIICXAIBAAKBgQCC5rXBOB2C3KMIx0CNv8Mv1+KfeD/I3/GIG7z06WGqX+wcN2H8
        Vn/AgO7q5xLf8oHIM7aUR6GEYpdniu65/Gq+cl9bOqOW013SxtZgXchyhhI/RTgx
        aBEWvl6ETGx19SrPoFmpLIQMCn+sCx4EmyA5rs/+5JFxAcr0IOQA3V+cMwIDAQAB
        AoGAGhFNoqPFJLlBck8TqObGPPHV/IHa2erW048Kiw8q1t3xTBEvXyvuluxWJ8tK
        c/R8Kd06f77gCFgF5KXDMqB0tR1r/odaahAnkhZZXJQpLDPK5xHycJkQo3ygYKOX
        0pv/fiwA03rY0jVbxNLteQ0C5iD+oY2TPOjF/OXGQxrl6gkCQQC8DRMIkRxHYIsD
        MArOcR/kMPGaWLVCmqqwmS0nzwVND3gaQ7ddXHeh5vkFUeLWO6827zhRu0oQlZQ2
        UDWBecOlAkEAsjM2VrwBjFR8RrDtjI+DfrAmZoXpGYgkvyy/Uh0SverFXCrxcWMm
        hRQTiUDxGAsQDQmzwiciaS2S/9ZU/yz49wJBAK+Q/U4oPGCfT+m/9rbags3GrCpP
        25q9T9Lkj3X1H5vcb2MGCbGOXNq5C5Dd7Iva2TDOkqQr2XA2VlOG+w1Qo0kCQAFR
        mbRYndZRBb5sLsXchgeY/B/EG+6BcGwtw5iy+GGJbD57XuAM8MsJU/+vCFm6Mzs1
        eCwWqA2JNc5I4sYkqg8CQDfStCITE6J897mwtk4PKyY02IffoC0Fm6ybgjoYV58l
        o48pTI2NayJHzm1S5Zp8ZTDrAqQJulqPOhWmnzJfhdE=
        -----END RSA PRIVATE KEY-----


        -----BEGIN PUBLIC KEY-----
        MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCC5rXBOB2C3KMIx0CNv8Mv1+Kf
        eD/I3/GIG7z06WGqX+wcN2H8Vn/AgO7q5xLf8oHIM7aUR6GEYpdniu65/Gq+cl9b
        OqOW013SxtZgXchyhhI/RTgxaBEWvl6ETGx19SrPoFmpLIQMCn+sCx4EmyA5rs/+
        5JFxAcr0IOQA3V+cMwIDAQAB
        -----END PUBLIC KEY-----
        */

        // Create transaction and sign it
        let mut transaction: Transaction = Transaction{
            sender: "MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCC5rXBOB2C3KMIx0CNv8Mv1+KfeD/I3/GIG7z06WGqX+wcN2H8Vn/AgO7q5xLf8oHIM7aUR6GEYpdniu65/Gq+cl9bOqOW013SxtZgXchyhhI/RTgxaBEWvl6ETGx19SrPoFmpLIQMCn+sCx4EmyA5rs/+5JFxAcr0IOQA3V+cMwIDAQAB".to_string(),
            receiver: "test".to_string(), 
            amount: "0".to_string(), 
            signature: vec![0]
        };
        sign(&mut transaction);
        assert_eq!(verify_transaction(&transaction), true);

    }

    /// Helper function for the test that signs a transaction with a pre-defined private key
    fn sign(transaction: &mut Transaction) {

        let private_key: &str = "-----BEGIN RSA PRIVATE KEY-----\n\
        MIICXAIBAAKBgQCC5rXBOB2C3KMIx0CNv8Mv1+KfeD/I3/GIG7z06WGqX+wcN2H8\n\
        Vn/AgO7q5xLf8oHIM7aUR6GEYpdniu65/Gq+cl9bOqOW013SxtZgXchyhhI/RTgx\n\
        aBEWvl6ETGx19SrPoFmpLIQMCn+sCx4EmyA5rs/+5JFxAcr0IOQA3V+cMwIDAQAB\n\
        AoGAGhFNoqPFJLlBck8TqObGPPHV/IHa2erW048Kiw8q1t3xTBEvXyvuluxWJ8tK\n\
        c/R8Kd06f77gCFgF5KXDMqB0tR1r/odaahAnkhZZXJQpLDPK5xHycJkQo3ygYKOX\n\
        0pv/fiwA03rY0jVbxNLteQ0C5iD+oY2TPOjF/OXGQxrl6gkCQQC8DRMIkRxHYIsD\n\
        MArOcR/kMPGaWLVCmqqwmS0nzwVND3gaQ7ddXHeh5vkFUeLWO6827zhRu0oQlZQ2\n\
        UDWBecOlAkEAsjM2VrwBjFR8RrDtjI+DfrAmZoXpGYgkvyy/Uh0SverFXCrxcWMm\n\
        hRQTiUDxGAsQDQmzwiciaS2S/9ZU/yz49wJBAK+Q/U4oPGCfT+m/9rbags3GrCpP\n\
        25q9T9Lkj3X1H5vcb2MGCbGOXNq5C5Dd7Iva2TDOkqQr2XA2VlOG+w1Qo0kCQAFR\n\
        mbRYndZRBb5sLsXchgeY/B/EG+6BcGwtw5iy+GGJbD57XuAM8MsJU/+vCFm6Mzs1\n\
        eCwWqA2JNc5I4sYkqg8CQDfStCITE6J897mwtk4PKyY02IffoC0Fm6ybgjoYV58l\n\
        o48pTI2NayJHzm1S5Zp8ZTDrAqQJulqPOhWmnzJfhdE=\n\
        -----END RSA PRIVATE KEY-----";

        println!("{}", private_key);

        // To rsa
        let priv_rsa = Rsa::private_key_from_pem(&private_key.as_bytes()).expect("Error converting pem to private key");
        let keypair = PKey::from_rsa(priv_rsa).unwrap();

        // Sign the data
        let data = format!("{}", transaction);
        let mut signer = Signer::new(MessageDigest::sha256(), &keypair).unwrap();
        signer.update(data.as_bytes()).unwrap();
        let signature = signer.sign_to_vec().unwrap();
        //println!("{}", String::from_utf8_lossy(&signature));
        transaction.signature = signature;
    }

}