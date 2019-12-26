extern crate rumqtt;
extern crate chrono;
use rumqtt::{MqttOptions, MqttClient, QoS, SecurityOptions};
use std::time::Duration;
use std::thread::sleep;
use std::env;
use std::io::Read;
use std::fs::File;
use chrono::prelude::*;

fn varvalue(key: &str) -> String {
    match env::var(key) {
        Ok(s) => return s,
        _ => panic!(format!("{} not set", key)),
    };
}

fn main() {
    let mqtt_host = varvalue("MQTT_HOST");
    let mqtt_port :u16 = varvalue("MQTT_PORT").parse().unwrap();
    
    // these don't have to be set
    let mqtt_ca = env::var("MQTT_CA");
    let mqtt_user = env::var("MQTT_USER");
    let mqtt_pass = env::var("MQTT_PASS");

    let every_s = varvalue("EVERY_SECONDS");
    let every_s :u64 = every_s.parse().expect("EVERY_SECONDS should be a number");
    
    let client_options = MqttOptions::new("time_service", mqtt_host, mqtt_port);
    let client_options = match mqtt_ca {
        Ok(ca) => {
	    let mut f = File::open(ca).unwrap();
	    let mut buffer = Vec::new();
	    f.read_to_end(&mut buffer).unwrap();
	    client_options.set_ca(buffer)
	},
        Err(_) => client_options
    };

    let client_options = match (mqtt_user, mqtt_pass) {
        (Ok(u), Ok(p)) => {
	    let security_options = SecurityOptions::UsernamePassword(u, p);
	    client_options.set_security_opts(security_options)
	},
        _ => client_options
    };

    let unix_epoch = Utc.ymd(1970, 1, 1).and_hms(0, 0, 0);

    let (mut client, _notifications)  = MqttClient::start(client_options).expect("can't start");
    loop {
        let t = Local::now();
        let e = t.signed_duration_since(unix_epoch);
        let seconds = e.num_seconds();
        let epoch_message = format!("{}", seconds);
        let rfc3339_message = t.to_rfc3339();
        client.publish("time/epoch", QoS::AtLeastOnce, false, epoch_message.into_bytes()).expect("publish failure");
        client.publish("time/rfc3339", QoS::AtLeastOnce, false, rfc3339_message.into_bytes()).expect("publish failure");
        sleep(Duration::new(every_s, 0));
    }
}
