use redis::{Commands, Connection, FromRedisValue, RedisError, RedisResult};
use std::fmt::Error;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

const SECCESS: &'static str = "[SECCESS OPERATION]";

pub struct Redis {
    conn: Connection,
}

impl Redis {
    pub fn new() -> Self {
        Self {
           conn: connection(),
        }
    }

    pub fn set(&mut self, key: &str, value: i32) -> &str {
        let _: () = self.conn.set(key, value).unwrap();
        SECCESS
    }

    pub fn get(&mut self, key: &str) -> redis::RedisResult<i32> {
        self.conn.get(key)
    }

    pub fn del(&mut self, key: &str) -> Result<(), RedisError> {
        self.conn.del(key)     
    }

    pub fn keys(&mut self) -> Result<Vec<String>, RedisError> {
        self.conn.keys("*")
    }

    pub fn set_words(&mut self) {
        if let Ok(lines) = read_lines("./words.txt") {
            for line in lines {
                if let Ok(word) = line {
                    self.set(word.as_str(), 1);
                }
            }
        }
    }

    pub fn write_word(&mut self, word: &str) {
        let file = OpenOptions::new().append(true).open("./words.txt");
        match file {
            Ok(mut file) => {
                let new_word = format!("\n{}", word);
                match file.write_all(new_word.as_bytes()) {
                    Ok(()) => {},
                    Err(e) => {
                        panic!("Error {}", e);
                    }
                }
                let _ = file.sync_all();
            },
            Err(e) => {
                panic!("Erorr {}", e);
            },
        }
    }

    pub fn write_words(&mut self, words: Vec<String>) {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("./words.txt");
        match file {
            Ok(mut file) => {
                for word in words.iter() {
                    let new_word = format!("\n{}", word);
                    match file.write_all(new_word.as_bytes()) {
                        Ok(()) => {},
                        Err(e) => {
                            panic!("Error {}", e);
                    }
                }
                let _ = file.sync_all();
                }
            },
            Err(e) => {
                panic!("Erorr {}", e);
            },
        }
    }
}

fn connection() -> Connection {
    match redis::Client::open("redis://127.0.0.1/") {
        Ok(client) => match client.get_connection() {
            Ok(conn) => return conn,
            Err(e) => panic!("Bad redis connection {}", e),
        },
        Err(e) => panic!("Bad redis connection {}", e),
    }
}

fn read_lines<T>(filename: T) -> io::Result<io::Lines<io::BufReader<File>>>
    where T: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
