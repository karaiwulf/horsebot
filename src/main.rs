extern crate daemonize;
extern crate serenity;
//extern crate syslog;
extern crate uuid;
extern crate regex;
use std::{fs, thread, time};
//use syslog::{Facility, Formatter3164};
use daemonize::Daemonize;
use uuid::Uuid;
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use regex::Regex;
use rand::distributions::{IndependentSample, Range};

struct Handler;
impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        let horsere = Regex::new(r".*[Hh][Oo][Rr][Ss][Ee].*").unwrap();
        if horsere.is_match(&msg.content) {
            let between = Range::new(10,2600);
            let mut rng = rand::thread_rng();
            thread::sleep(time::Duration::from_secs(between.ind_sample(&mut rng)));
            if let Err(e) = msg.channel_id.say(&ctx.http, "horse") {
                println!("Error sending message: {:?}", e);
            }
        }
    }
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
fn horsed(token: String) {
    let mut client = Client::new(&token, Handler)
        .expect("Err creating client");
    if let Err(e) = client.start() {
        println!("Client Error: {:?}", e);
    }
}
fn getkey() -> String {
    let token = match fs::read_to_string("/etc/horsed/key") {
        Ok(retme) => retme,
        Err(e) => panic!("unable to read key: {:?}", e),
    };
    token
}
fn main() {
    let uniq = Uuid::new_v4();
    let dir = "/tmp/horsed/".to_string() + &uniq.to_simple().to_string() + "/";
    let mut out: String = "".to_string();
    out.push_str(&dir);
    out.push_str("horsed.out");
    let mut err: String = "".to_string();
    err.push_str(&dir);
    err.push_str("horsed.err");
    let mut pidf: String = "".to_string();
    pidf.push_str(&dir);
    pidf.push_str("horsed.pid");
    match fs::create_dir_all(&dir) {
        Ok(_) => {},
        Err(e) => panic!("unable to create /tmp: {:?}", e),
    };
    let stdout = fs::File::create(&out).unwrap();
    let stderr = fs::File::create(&err).unwrap();
    let token = getkey();
    let daemonize = Daemonize::new()
        .pid_file(&pidf)
        .chown_pid_file(true)
        .working_directory("/tmp")
        .user("nobody")
        .group("daemon")
        .group(2)
        .umask(0o777)
        .stdout(stdout)
        .stderr(stderr)
        .exit_action(|| println!("{}, {}!", 
                                 "Now to be a real daemon", 
                                 "fork myself and kill my parent"));
    match daemonize.start() {
        Ok(_) => horsed(token), 
        Err(e) => eprintln!("Error, {}", e), 
    }
}
