// src/utils/redis_stream.rs

use redis::{Connection,Commands, RedisError};
use redis::streams::StreamReadOptions;
use serde::{Deserialize, Serialize};
use serde_json;


#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub event_type: String,
    pub data: serde_json::Value,
}

pub async fn read_events_perplexity(client: &mut Connection, stream_key: &str, count: usize) -> Result<Vec<Event>, RedisError> {
    // Read events from the stream.  We're using XREAD with BLOCK to wait for events.
    let opts = StreamReadOptions::default().count(count);
    let events_raw: Vec<Vec<(String, Vec<(String, String)>)>> = client.xread_options(&[stream_key], &["0-0"], &opts)?;

    let mut events: Vec<Event> = Vec::new();
    for stream_events in events_raw {
        for (_stream_id, event_data) in stream_events {
            // Assuming event_data is a vector of key-value pairs
            // and the event JSON is in the 'event' key
            let event_json = event_data.iter().find(|&(key, _)| key == "event");
            if let Some((_key, event_json)) = event_json {
                let event: Event = serde_json::from_str(event_json).expect("Failed to deserialize event");
                events.push(event);
            }
        }
    }

    Ok(events)
}

pub async fn read_events(con: &mut Connection, stream_key: &str, count: usize) -> Result<Vec<Event>, RedisError> {
    let opts = StreamReadOptions::default().count(count);
    let raw_events: Vec<(String, Vec<(String, Vec<(String, String)>)>)> = con.xread_options(&[stream_key], &["0-0"], &opts)?;

    let mut events: Vec<Event> = Vec::new();
    for (_stream, messages) in raw_events {
        for (_id, fields) in messages {
            if let Some((_, event_json)) = fields.into_iter().find(|(key, _)| key.len() > 0) {
                let event: Event = serde_json::from_str(&event_json).expect("Invalid JSON");
                events.push(event);
            }
        }
    }
    Ok(events)
}




// pub async fn read_events(client: &mut Connection, stream_key: &str, count: usize) -> Result<Vec<Event>, RedisError> {
//     // Read events from the stream.  We're using XREAD with BLOCK to wait for events.
//     let opts = StreamReadOptions::default().count(count);
//     let events_raw: Vec<Vec<Vec<String>>> = client.xread_options(&[stream_key], &[0-0], &opts)?;

//     let mut events: Vec<Event> = Vec::new();
//     for stream_events in events_raw {
//         for event in stream_events {
//             //0th element is the stream id, the rest are event data in key-value pairs
//             let event_json = &event[1];
//             let event: Event = serde_json::from_str(&event_json).expect("Failed to deserialize event");
//             events.push(event);
//         }
//     }

//     Ok(events)
// }

pub async fn write_event(client: &mut Connection, stream_key: &str, event: &Event) -> Result<(), RedisError> {
    let event_json = serde_json::to_string(event).expect("Failed to serialize event"); //Panic if serialization fails. Consider a Result for better error handling.

    let _: () = client.xadd(stream_key, "*", &[("event", &event_json)])?;
    Ok(())
}


/*
// Function to get one JSON object by ID
pub fn get_one(
    con: &mut redis::Connection,
    id: &str,
    stream: &str, // The stream to read from
) -> Result<Option<serde_json::Value>, redis::RedisError> {

    let opts = StreamReadOptions::default()
    .count(1);
    let results: StreamReadReply =
        con.xread_options(&[&stream], &[&id], &opts)
        .expect("read");

    let json_string = results.keys[0].ids[0].map.get("json").unwrap();
    let json_value: serde_json::Value = from_str(json_string).unwrap();
    Ok(Some(json_value));

    // for redis::streams::StreamKey { key, ids} in results.keys {
    //     println!("StreamKey: {:?}", key);
    //     for redis::streams::StreamId  { id, map } in ids {
    //         println!("StreamId: {:?}", id);
    //         for (field, value) in map {
    //             println!("Field: {:?} Value: {:?}", field, value);
    //         }
    //     }
    // }

}


// Function to write one JSON object with a specific ID and key
pub fn write_one(
    con: &mut redis::Connection,
    key: &str, // The key for the JSON object
    json_value: serde_json::Value,
    stream: &str, // The stream to write to
) -> Result<(), redis::RedisError> {
    let json_string = to_string(&json_value)?;
    let _: String = con.xadd(stream, "*", &[("json", json_string)])?;
    Ok(())
}


*/
