extern crate jack;
extern crate uuid;
extern crate ws;

use jack::prelude as j;
use jack::traits::*;

use std::collections::HashMap;
use std::io;
use std::sync::Arc;
use std::sync::RwLock;

use uuid::Uuid;

use ws::{listen, Handler, Sender, Result, Message, Handshake, CloseCode, Error};

struct Server<'a> {
    out: Sender,
    id: String,
    clients: Arc<RwLock<&'a HashMap<String, &'a Server<'a>>>>,
}

impl<'a> Handler for Server<'a> {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        println!("open");
        Ok(())
    }
    fn on_message(&mut self, _: Message) -> Result<()> {
        println!("message");
        Ok(())
    }
    fn on_close(&mut self, _: CloseCode, _: &str) {
        println!("close");
    }
    fn on_error(&mut self, _: Error) {
        println!("error");
    }
}

/// sudo jack_connect SuperCollider:out_1 jack-web-audio:jwa_in_l
/// sudo jack_connect SuperCollider:out_2 jack-web-audio:jwa_in_r
fn main() {
    let (client, _status) = j::Client::open("jack-web-audio", j::client_options::NO_START_SERVER)
        .unwrap();
    let in_l = client.register_port("jwa_in_l", j::AudioInSpec::default()).unwrap();
    let in_r = client.register_port("jwa_in_r", j::AudioInSpec::default()).unwrap();
    let process = move |_: &j::WeakClient, ps: &j::ProcessScope| -> jack::JackControl {
        let l = j::AudioInPort::new(&in_l, ps);
        let r = j::AudioInPort::new(&in_r, ps);
        j::JackControl::Continue
    };
    let handler = j::ProcessHandler::new(process);
    let active_client = client.activate(handler).unwrap();

    let clients = HashMap::new();
    let mut shared_clients = Arc::new(RwLock::new(&clients));
    listen("0.0.0.0:8003", |out| {
        let id = Uuid::new_v4().hyphenated().to_string();
        Server{ out: out, clients: shared_clients.clone(), id: id }
    }).unwrap();
}
