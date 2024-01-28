pub trait Communication {
    fn broadcast(&mut self, p: &Packet);
}

#[derive(Clone)]
pub struct Packet {
    sender_name: String,
    sender_cid: Option<u64>,
    message: String,
}

impl Packet {}

// General Code

pub struct Vcp {
    /// The Cord Id. `None` means not assigned. And 0 is the first device.
    c_id: Option<u64>,
    pub debug_name: String,
    pub outgoing_msgs: Vec<Packet>,

    predecessor: Option<u64>,
    successor: Option<u64>,
}

impl Vcp {
    pub fn new(is_first: bool) -> Self {
        let id = if is_first { None } else { Some(0) };
        Vcp {
            c_id: id,
            debug_name: String::from(""),
            outgoing_msgs: Vec::new(),
            successor: None,
            predecessor: None,
        }
    }

    /// Method is called, when a new message is received.
    pub fn receive(&mut self, packet: &Packet) {
        println!(
            "{}: received '{}' from {}",
            self.debug_name, packet.message, packet.sender_name
        );

        //self.outgoing_msgs.push(Packet {message: });
    }

    pub fn send_init_message(&mut self) {
        let p = Packet {
            message: String::from("Init"),
            sender_name: self.debug_name.clone(),
            sender_cid: self.c_id,
        };
        self.send(&p);
    }

    fn send(&mut self, packet: &Packet) {
        self.outgoing_msgs.push(packet.clone());
    }
}
