#[allow(unused_variables)]
#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}
#[derive(Debug)]
enum StatusMessage {
    Ok,
}
#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}
type Message = String;
struct Groundstation {}
impl Groundstation {
    fn send(&self, to: &mut CubeSat, msg: Message) {
        to.mailbox.messages.push(msg);
    }
}
impl CubeSat {
    fn recv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
}
fn check_status(sat_id: &CubeSat) {
    println!("{:?}: {:?}", sat_id.id, StatusMessage::Ok);
}

fn main() {
    let base = Groundstation {};
    let sat_a = CubeSat {
        id: 0,
        mailbox: Mailbox { messages: vec![] },
    };
    let mut sat_b = CubeSat {
        id: 1,
        mailbox: Mailbox { messages: vec![] },
    };
    let sat_c = CubeSat {
        id: 2,
        mailbox: Mailbox { messages: vec![] },
    };
    base.send(&mut sat_b, Message::from("hello there!"));
    println!("{:?}", sat_b);

    check_status(&sat_a);
    check_status(&sat_b);
    check_status(&sat_c);

    //waiting
    check_status(&sat_a);
    check_status(&sat_b);
    check_status(&sat_c);
}
