#[allow(unused_variables, dead_code)]
#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}
impl CubeSat {
    fn recv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
}
#[derive(Debug)]
enum StatusMessage {
    Ok,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}
impl Mailbox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }
    fn deliver(&mut self, receipent: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == receipent.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }
        None
    }
}

#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

struct Groundstation {}

impl Groundstation {
    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat {
            id: sat_id,
            mailbox: Mailbox { messages: vec![] },
        }
    }
    fn send(&self, to: &mut CubeSat, msg: Message) {
        to.mailbox.messages.push(msg);
    }
}

fn check_status(sat_id: &CubeSat) {
    println!("{:?}: {:?}", sat_id.id, StatusMessage::Ok);
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

fn main() {
    let base = Groundstation {};
    let sat_ids = fetch_sat_ids();
    for sat_id in sat_ids {
        let mut sat = base.connect(sat_id);
        base.send(&mut sat, Message::from("hello"));
    }
}
