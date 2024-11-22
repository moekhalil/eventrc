#![allow(dead_code)]

use redis::{Commands, FromRedisValue};

mod utils;
use utils::redis_stream;
use utils::redis_docs;

extern crate redis;


// main function to write string:string key value pair to redis
#[tokio::main]
async fn main() {
    let client: redis::Client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();

    print_redis_docs(&mut con).await;
    create_event_in_stream(&mut con).await;
    print_stream_events(&mut con).await;
}

async fn create_event_in_stream(con: &mut redis::Connection) {
    let stream_key = "mystream2";
    let event = redis_stream::Event {
        event_type: "my_event".to_string(),
        data: serde_json::json!({
            "name": "Moe",
            "age": 35,
        }),
    };
    redis_stream::write_event(con, stream_key, &event).await.unwrap();
}

async fn print_stream_events(con: &mut redis::Connection) {
    let stream_key = "mystream2";
    let count = 10;
    let mystream_results = redis_stream::read_events(con, stream_key, count);
    let results = mystream_results.await.unwrap();
    println!("Printing events");
    for result in results {
        println!("{:?}", serde_json::to_string(&result).unwrap());
    }
}

async fn print_redis_docs(con: &mut redis::Connection) {
    let key = "settings:my_setting";
    let value = serde_json::json!({
        "name": "My Setting",
        "value": 42,
    });
    redis_docs::set_one(con, key, value).unwrap();
    let result = redis_docs::get_one(con, key).unwrap();
    println!("Setting: {:?}", result);
}


// function to write a key value pair to redis that is a string:hashmap
// good for storing json objects
fn xadd_to_redis() {

    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();

    let res: Result<String, redis::RedisError> = con.xadd(
        "mystream",
        "*",
        &[("name", "moe"), ("gender", "male")]
    );
    match res {
        Ok(_) => println!("{} => {}", "mystream", "Saved!"),
        Err(e) => println!("Error: {}", e),
    }

    // read the stream
    let xread_result = con.xread(&["mystream"], &["0"]).unwrap();

    // parse the result
    let res: Vec<(String, Vec<(String, Vec<(String, String)>)>)> =
        FromRedisValue::from_redis_value(&xread_result).unwrap();

    // print the result
    for (stream_name, stream_body) in res {
        println!("Stream: {:?}", stream_name);
        for (document_id, fields) in stream_body {
            println!("Document ID: {:?}", document_id);
            for (field_name, field_value) in fields {
                println!("Field Name: {:?}", field_name);
                println!("Field Value: {:?}", field_value);
            }
            println!("=====================");
        }
    }
}

struct ValueKey <'a> {
    key: &'a str,
    // define a optional value property
    #[allow(dead_code)]
    value: Option<String>,
}
