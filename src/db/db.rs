use redis::{Commands, Connection, FromRedisValue, RedisError, RedisResult, Value};
use std::fmt::Error;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use postgres::{Client, NoTls};
use postgres::Error as PostgresError;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

const SECCESS: &'static str = "[SECCESS OPERATION]";

pub struct Redis {
    conn: Connection,
}

pub struct Postgres {
    conn: Client,
}

#[derive(Serialize, Deserialize)]
struct Wallet {
    wallet: String,
}

impl Redis {
    pub fn new() -> Self {
        Self {
           conn: connection_redis(),
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

impl Postgres {
    pub fn new() -> Self {
        Self {
            conn: Client::connect("postgres://ton:ton@127.0.0.1:5432/ton", NoTls).unwrap(),
        }
    }
    pub fn create(&mut self) -> Result<(), PostgresError> {
        let bar = 1i32;
        let baz = true;

        self.conn.execute("CREATE TABLE transactions (
            source VARCHAR (48) NOT NULL,
            hash VARCHAR (50) UNIQUE NOT NULL,
            value VARCHAR (50) NOT NULL,
            comment VARCHAR (50))
        ", &[&bar, &baz])?;

        self.conn.execute("CREATE TABLE users (
            id INTEGER UNIQUE NOT NULL,
            username VARCHAR (33),
            first_name VARCHAR (300),
            wallet VARCHAR (50) DEFAULT none)
        ", &[&bar, &baz])?;

        Ok(())
    }

    pub fn add_v_transaction(&mut self, source: String, hash: String, value: i32, comment: String) -> Result<(), PostgresError> {
        self.conn.execute("INSERT INTO transactions (source, hash, value, comment) VALUES ($1, $2, $3, $4)"
        ,&[&source, &hash, &value, &comment])?;

        Ok(())
    }

    pub fn check_transaction(&mut self, hash: String) -> Result<(), PostgresError> {
        self.conn.execute("SELECT hash FROM transactions WHERE hash = $1"
        , &[&hash])?;

        Ok(())
    }

    pub fn check_user(&mut self, user_id: i32, username: String, first_name: String) -> Result<(), PostgresError> {
        match self.conn.execute("SELECT id FROM users WHERE id = $1"
        , &[&user_id]) {
            Ok(_) => (),
            Err(_) => {
                self.conn.execute("INSERT INTO users (id, username, first_name) VALUES ($1, $2, $3)"
                , &[&user_id, &username, &first_name])?;
            }
        }

        Ok(())
    }

    pub fn v_wallet(&mut self, user_id: i32, wallet: String) -> Result<(), PostgresError> {
        match self.conn.execute("SELECT wallet FROM users WHERE id = $1"
        , &[&user_id]) {
            Ok(_) => (),
            Err(_) => {
                self.conn.execute("UPDATE users SET wallet = $1 WHERE id = $2"
                , &[&wallet, &user_id])?;
            }
        }

        Ok(())
    }

    pub fn get_user_wallet(&mut self, user_id:i32) -> Result<String, PostgresError> {
        match self.conn.query_one("SELECT wallet FROM users WHERE id = $1"
        , &[&user_id]) {
            Ok(row) => {
                let wallet = Wallet {
                    wallet: row.get(0),
                };

                return Ok(wallet.wallet)
            }
            Err(err) => {
                return Err(err)
            },
        }

    }

    // pub fn get_user_payment(&mut self, user_id: i32) -> Result<(), String> {
    //     match self.get_user_wallet(user_id) {
    //         Ok(wallet) => {
    //             let result = self.conn.query("SELECT * FROM transactions WHERE source = $1"
    //             , &[&wallet]);
    //             let tdict: HashMap<String, String> = HashMap::new();
    //             let tlist = Vec::new();

    //             for transaction in result {
    //                 let w = transaction.get(0).unwrap();
    //                 tdict.insert(String::from("value"), transaction.get(0));
    //             }
    //         },
    //         Err(err) => {
    //             return Result<(), "You haven't wallet">
    //         }
    //     }

    //     Ok(())
    // }
}

fn connection_redis() -> Connection {
    match redis::Client::open("redis://127.0.0.1/") {
    // match redis::Client::open("redis://redis/") { 
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
