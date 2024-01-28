pub trait Communication {
    fn broadcast(&mut self, p: &Packet);
}

#[derive(Clone, Debug)]
pub enum Message {
    Init,
    Ack { orig_sender_cit: u64 },
    Text(String),
}

#[derive(Clone, Debug)]
pub struct Packet {
    sender_name: String,
    sender_cid: Option<u64>,
    message: Message,
}

impl Packet {
    pub fn new(src: &Vcp, mesage: Message) -> Self {
        Packet {
            sender_name: src.debug_name.clone(),
            sender_cid: src.c_id,
            message: mesage,
        }
    }
}

// General Code

pub struct Vcp {
    /// The Cord Id. `None` means not assigned. And 0 is the first device.
    pub c_id: Option<u64>,
    pub debug_name: String,
    /// All messages that should be sent
    pub outgoing_msgs: Vec<Packet>,

    pub predecessor: Option<u64>,
    pub successor: Option<u64>,
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
            "{}: received '{:?}' from {}",
            self.debug_name, packet.message, packet.sender_name
        );

        match packet.message {
            Message::Init => self.send(&Packet::new(
                self,
                Message::Ack {
                    orig_sender_cit: packet.sender_cid.expect("No cid was set for init message"),
                },
            )),
            Message::Ack { orig_sender_cit } => (),
            Message::Text(_) => todo!(),
        }

        if let Some(cid) = packet.sender_cid {
            // TODO very basic cord fiunc
            self.successor = Some(cid);
        }
        //self.outgoing_msgs.push(Packet {message: });
    }

    pub fn send_init_message(&mut self) {
        self.c_id = Some(0);
        let p = Packet {
            message: Message::Init,
            sender_name: self.debug_name.clone(),
            sender_cid: self.c_id,
        };
        self.send(&p);
    }

    fn send(&mut self, packet: &Packet) {
        println!("Sending {:?}", packet);
        self.outgoing_msgs.push(packet.clone());
    }
}
