#[macro_use]
extern crate serde_derive;
extern crate redis;
extern crate serde;
extern crate serde_json;
use redis::Commands;

use serde_json::value::Value;

use std::collections::BTreeMap;
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
    pub class: String,
    pub args: Vec<Value>,
    pub retry: i64,
    pub queue: String,
    pub jid: String,
    pub created_at: u64,
    pub enqueued_at: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    pub url: Url,
    pub document: String,
    pub headers: BTreeMap<String, String>,
    pub status: String,
    pub urls: Vec<Url>,
}

fn do_something() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    let k: Option<String> = con.lindex("development:queue:maman", 0)?;
    let k1 = k.unwrap();

    let k2: Job = serde_json::from_str(&k1).unwrap();
    // println!("Job Deserialize: {:?}", k2);

    // Now get at the args field inside the Job struct
    let args = k2.args;

    // Each arg is a Page
    // I do not believe the Pages are json, only the above Jobs
    // So that is why you can not serde_json::from_str here
    for arg in args.iter() {
        let k3 = arg.as_object().unwrap();
        // This prints the underlying Page
        // println!("{:?}", k3);

        // Only grab the url and the document from the Page
        // Turns out the Page fields are a set of tuples
        for key in k3.iter() {
            // The next line shows the actual underlying tuples
            // println!("{:?}", key);
            // if you want to see all of the fields/ keys
            // uncomment the next line
            // println!("{:?}", key.0);

            if key.0 == "document" {
                println!("{:?}", key.1)
            }

            if key.0 == "url" {
                println!("{:?}", key.1)
            }
        }
    }

    Ok(())
}

fn main() {
    println!("These are the maman keys of interest!");
    do_something();
}
