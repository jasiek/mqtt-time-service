extern crate rumqtt;
extern crate chrono;
use rumqtt::{MqttOptions, MqttClient, QoS};
use std::time::Duration;
use std::thread::sleep;
use std::env;
use chrono::prelude::*;

fn main() {
    let mqtt_host = env::var("MQTT_HOST").expect("MQTT_HOST not set");
    let mqtt_port = env::var("MQTT_PORT").expect("MQTT_PORT not set");
    let every_s = env::var("EVERY_SECONDS").expect("EVERY_SECONDS not set");
    let every_s :u64 = every_s.parse().expect("EVERY_SECONDS should be a number");
    
    let client_options = MqttOptions::new().set_broker(&format!("{}:{}", mqtt_host, mqtt_port));
    let unix_epoch = Utc.ymd(1970, 1, 1).and_hms(0, 0, 0);

    let mut request = MqttClient::start(client_options, None).expect("can't start");
    loop {
        let t = Local::now();
        let e = t.signed_duration_since(unix_epoch);
        let seconds = e.num_seconds();
        let message = format!("{}", seconds);
        println!("{}", seconds);
        request.publish("time/epoch", QoS::Level1, message.into_bytes()).expect("publish failure");
        sleep(Duration::new(every_s, 0));
    }
}
