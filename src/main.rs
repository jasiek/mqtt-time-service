extern crate rumqtt;
extern crate chrono;
use rumqtt::{MqttOptions, MqttClient, QoS};
use std::time::Duration;
use std::thread::sleep;
use std::env;
use chrono::prelude::*;

fn varvalue(key: &str) -> String {
    match env::var(key) {
        Ok(s) => return s,
        _ => panic!(format!("{} not set", key)),
    };
}

fn main() {
    let mqtt_host = varvalue("MQTT_HOST");
    let mqtt_port = varvalue("MQTT_PORT");

    let every_s = varvalue("EVERY_SECONDS");
    let every_s :u64 = every_s.parse().expect("EVERY_SECONDS should be a number");
    
    let client_options = MqttOptions::new().set_broker(&format!("{}:{}", mqtt_host, mqtt_port)).set_should_verify_ca(true).set_ca("/opt/local/share/curl/curl-ca-bundle.crt");
    let unix_epoch = Utc.ymd(1970, 1, 1).and_hms(0, 0, 0);

    let mut request = MqttClient::start(client_options, None).expect("can't start");
    loop {
        let t = Local::now();
        let e = t.signed_duration_since(unix_epoch);
        let seconds = e.num_seconds();
        let epoch_message = format!("{}", seconds);
        let rfc3339_message = t.to_rfc3339();
        println!("{}", rfc3339_message);
        request.publish("time/epoch", QoS::Level1, epoch_message.into_bytes()).expect("publish failure");
        request.publish("time/rfc3339", QoS::Level1, rfc3339_message.into_bytes()).expect("publish failure");
        sleep(Duration::new(every_s, 0));
    }
}
