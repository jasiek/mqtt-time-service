extern crate rumqtt;
use rumqtt::{MqttOptions, MqttClient, QoS};
use std::time::{SystemTime, Duration, UNIX_EPOCH};
use std::thread::sleep;
use std::env;

fn main() {
    let mqtt_host = env::var("MQTT_HOST").expect("MQTT_HOST not set");
    let mqtt_port = env::var("MQTT_PORT").expect("MQTT_PORT not set");
    let every_s = env::var("EVERY_SECONDS").expect("EVERY_SECONDS not set");
    let every_s :u64 = every_s.parse().expect("EVERY_SECONDS should be a number");
    
    let client_options = MqttOptions::new().set_broker(&format!("{}:{}", mqtt_host, mqtt_port));

    let mut request = MqttClient::start(client_options, None).expect("can't start");
    loop {
        let t = SystemTime::now();
        let e = t.duration_since(UNIX_EPOCH).expect("time went backwards");
        let seconds = e.as_secs();
        let message = format!("{}", seconds);
        println!("{}", seconds);
        request.publish("time/epoch", QoS::Level1, message.into_bytes()).expect("publish failure");
        sleep(Duration::new(every_s, 0));
    }
}
