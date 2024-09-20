extern crate redis;


use redis::{Commands, FromRedisValue};

#[allow(dead_code)]
fn __main2() {
    let doc_key = "test_doc_2";
    let doc_content = "test_doc_2: Hello, world! From Redis!";

    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();

    let res: Result<String, redis::RedisError> = con.set(doc_key, doc_content);
    match res {
        Ok(_) => println!("{} => {}", doc_key, "Saved!"),
        Err(e) => println!("Error: {}", e),
    }

    let result: String = con.get(doc_key).unwrap();

    println!("{} => {}", doc_key, result);

}

#[allow(dead_code)]
fn get_value_for_key(key: ValueKey) {

    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();

    let result: String = con.get(key.key).unwrap();

    println!("{} => {}", key.key, result);

}

#[allow(dead_code)]
fn get_all_keys() {

    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();

    let keys: Vec<String> = con.keys("*").unwrap();

    println!("Keys: {:?}", keys);
}

#[allow(dead_code)]
fn xadd_to_redis() {

    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();

    let res: Result<String, redis::RedisError> = con.xadd("mystream", "*", &[("name", "moe"), ("gender", "male")]);
    match res {
        Ok(_) => println!("{} => {}", "mystream2", "Saved!"),
        Err(e) => println!("Error: {}", e),
    }

    // print the stream
    let res:  Vec<Vec<(String, Vec<Vec<(String, Vec<(String, String)>)>>)>> = FromRedisValue::from_redis_value(&con.xread(&["mystream"], &["0"]).unwrap()).unwrap();
    res.iter().for_each(|x| {
        x.iter().for_each(|stream| {
            let stream_name = stream.0.as_str();
            println!("Stream: {:?}", stream_name);

            let stream_body = &stream.1;
            stream_body.iter().for_each(|message| {
                message.iter().for_each(|doc| {
                    let document_id = doc.0.as_str();
                    println!("Document ID: {:?}", document_id);

                    doc.1.iter().for_each(|field| {
                        let field_name = field.0.as_str();
                        println!("Field Name: {:?}", field_name);

                        let field_value = field.1.as_str();
                        println!("Field Value: {:?}", field_value);
                    });

                    print!("=====================\n");
                });
            });
        });
    });
    println!("Stream: {:?}", res);
}

struct ValueKey <'a> {
    key: &'a str,
    // define a optional value property
    #[allow(dead_code)]
    value: Option<String>,
}

fn main() {
    xadd_to_redis();
    get_all_keys();
    let dbkey = ValueKey { key: "test_doc_2", value: None };
    get_value_for_key(dbkey)
}
