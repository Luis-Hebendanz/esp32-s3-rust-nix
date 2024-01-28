use std::collections::{HashMap, HashSet};

pub trait Communication {
    fn broadcast(&mut self, p: &Packet);
}

#[derive(Clone, Debug)]
pub enum Message {
    Init,
    AckInit { orig_sender_cit: u64 },
    Hello(NeighborInfo),
    SendNewPosition { old_position: CordId },
    Text(String),
}

#[derive(Clone, Debug)]
/// A Packet that will be send over the air
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

type CordId = u64;

#[derive(Clone, Debug, Copy)]
pub struct NeighborInfo {
    predecessor: Option<CordId>,
    successor: Option<CordId>,
}

pub struct Vcp {
    /// The Cord Id. `None` means not assigned. And 0 is the first device.
    pub c_id: Option<CordId>,
    pub debug_name: String,
    /// All messages that should be sent
    pub outgoing_msgs: Vec<Packet>,

    pub predecessor: Option<CordId>,
    pub successor: Option<CordId>,

    pub neighbors: HashMap<CordId, NeighborInfo>,
}

impl Vcp {
    pub fn new(is_first: bool) -> Self {
        let id = if is_first { Some(0) } else { None };
        Vcp {
            c_id: id,
            debug_name: String::from(""),
            outgoing_msgs: Vec::new(),
            successor: None,
            predecessor: None,
            neighbors: HashMap::new(),
        }
    }

    fn set_my_position(&mut self) {
        const S: CordId = 0;
        const E: CordId = 100;
        const I: CordId = 10;
        let mut p_temp: CordId = 0;
        for (cid, neighbor) in self.neighbors.iter() {
            if *cid == S {
                if neighbor.successor.is_none() {
                    p_temp = E;
                } else if neighbor.successor == Some(E) {
                    p_temp = (S + E) / 2;
                } else {
                    p_temp = neighbor.successor.unwrap() - I * (neighbor.successor.unwrap() - *cid);
                }
                self.c_id = Some(p_temp);
                self.predecessor = Some(*cid);
                self.send_new_position_to_neighbor(*cid, p_temp);
                break;
            } else {
                todo!()
            }
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
                Message::AckInit {
                    orig_sender_cit: packet.sender_cid.expect("No cid was set for init message"),
                },
            )),
            Message::AckInit { orig_sender_cit } => (),
            Message::Text(_) => todo!(),
            Message::Hello(neigh) => {
                self.neighbors.insert(
                    packet.sender_cid.expect("Expected that CID is set"),
                    neigh.clone(),
                );
            }
            Message::SendNewPosition { old_position } => todo!(),
        }

        if let Some(cid) = packet.sender_cid {
            // TODO very basic cord fiunc
            self.successor = Some(cid);
        }
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

    /// Function that HAS to be called periodically
    pub fn timer_call(&mut self) {
        if self.c_id.is_none() {
            // TODO
            //self.set_my_position();
        } else {
            self.send(&Packet::new(
                &self,
                Message::Hello(NeighborInfo {
                    predecessor: self.predecessor,
                    successor: self.successor,
                }),
            ));
        }
    }

    fn send_new_position_to_neighbor(&self, cid: u64, p_temp: u64) {
        todo!();
    }
}
