use clap::{load_yaml, App};
use nats;
use std::time::Duration;

const DEFAULT_HOST: &str = "127.0.0.1";

fn main() {
    // the cli argument in a YAML file
    let yaml = load_yaml!("../cli.yaml");
    let matches = App::from(yaml).get_matches();

    // Gets host value provide by the user or use the default value
    let host = matches.value_of("host").unwrap_or(DEFAULT_HOST);
    let nc = nats::connect(host).unwrap();
    println!("connected to NATS server (host {})", host);

    let subject = matches.value_of("SUBJECT").unwrap();
    let message = matches.value_of("MESSAGE").unwrap_or("");

    match matches.value_of("VERB").unwrap() {
        "SUBSCRIBE" => subscribe(&nc, &subject),
        "PUBLISH" => publish(&nc, &subject, &message),
        "REQUEST" => unimplemented!(),
        _ => println!("unknown verb !"),
    }
}

fn subscribe(nc: &nats::Connection, subject: &str) {
    let sub = nc.subscribe(subject).unwrap();
    println!("subscribed to subject {}", subject);
    let mut cnt = 0;
    for msg in sub.messages() {
        println!("[{}] {}", cnt, &msg);
        cnt += 1;
    }
}

fn publish(nc: &nats::Connection, subject: &str, message: &str) {
    if message.len() == 0 {
        println!("no valid message provided")
    } else {
        nc.publish(&subject, &message).unwrap();
        std::thread::sleep(Duration::from_millis(100));
        println!("message published")
    }
}
