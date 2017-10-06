//use serde::de::Deserialize;
//use serde::ser::Serialize;

extern crate serde;
extern crate serde_json;



use serde_json::{Deserializer, Value, from_reader, from_slice, from_str, from_value,
                 to_string, to_string_pretty, to_value, to_vec, to_writer};


pub fn run_me() {

    // from https://now.httpbin.org/
    let s = r#"{"now": {"epoch": 1507283966.5156896, "slang_date": "today", "slang_time": "now", "iso8601": "2017-10-06T09:59:26.515690Z", "rfc2822": "Fri, 06 Oct 2017 09:59:26 GMT", "rfc3339": "2017-10-06T09:59:26.51Z"}, "urls": ["/", "/docs", "/when/:human-timestamp", "/parse/:machine-timestamp"]}"#;

    //let s = r#"{"user-agent": "Mozilla/5.0 (X11; Linux x86_64; rv:55.0) Gecko/20100101 Firefox/55.0"}"#;

    let prsd:Value = serde_json::from_str(s).unwrap();
    //println!("{:?}", prsd["user-agent"]);
    println!("{:?}", prsd["now"]["iso8601"]);


}