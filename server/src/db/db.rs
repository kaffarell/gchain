use std::{env, fs};
use leveldb::database::Database;
use leveldb::iterator::Iterable;
use leveldb::kv::KV;
use leveldb::options::{Options, WriteOptions, ReadOptions};
use crate::blockchain::block::Block;


pub fn create_database() -> Database<i32> {
    let mut dir = env::current_dir().unwrap();
    dir.push("chain");

    let path_buf = dir.clone();
    fs::create_dir_all(dir).unwrap();

    let path = path_buf.as_path();
    let mut options = Options::new();
    options.create_if_missing = true;

    // Create Database
    let database = match Database::open(path, options) {
        Ok(db) => {db},
        Err(e) => {panic!("Failed to open database: {:?}", e)}
    };
    return database;
}

pub fn get(database: &Database<i32>, index: i32) -> Option<Block> {
    // TODO: Fix index here, should be i64
    // Read from database
    let read_opts = ReadOptions::new();
    let res = database.get(read_opts, index);
    // res is a result so we have to check if it is Ok or Err
    match res {
        Ok(data) => {
            // data is an Option so we check if it is empty or not
            match data {
                Some(data) => {
                    // If we get some data convert the u8 vec to a string
                    let string: String = String::from_utf8(data).expect("Error converting database result (u8 vec) to string");
                    // Convert json string to Block struct
                    return Some(serde_json::from_str::<Block>(&string).unwrap());
                }
                None => {
                    return None;
                }

            }
        }
        Err(e) => {panic!("Failed to read from database: {:?}", e)}
    };

}

pub fn put(database: &Database<i32>, block: &Block) {
    // Write to database
    let write_ops = WriteOptions::new();
    let string = serde_json::to_string(block).unwrap();
    match database.put(write_ops, get_total_height(&database) as i32, string.as_bytes()) {
        Ok(_) => {()},
        Err(e) => {panic!("Failed to write to database: {:?}", e)}
    };
}

pub fn get_total_height(database: &Database<i32>) -> i64 {
    let read_opts = ReadOptions::new();
    let iter = database.iter(read_opts);
    return iter.count() as i64; 
}