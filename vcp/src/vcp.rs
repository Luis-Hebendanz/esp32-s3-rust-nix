use std::collections::{HashMap, HashSet};

pub trait Communication {
    fn broadcast(&mut self, p: &Packet);
}

#[derive(Clone, Debug)]
pub enum Message {
    Hello(NeighborInfo),
    SendUpdatePredecessor { new_position: CordId },
    SendUpdateSuccessor { new_position: CordId },
    Text(String),
}

#[derive(Clone, Debug)]
pub enum Receiver {
    Broadcast,
    Unicast(CordId),
}

#[derive(Clone, Debug)]
/// A Packet that will be send over the air
pub struct Packet {
    receiver: Receiver,
    sender_name: String,
    sender_cid: Option<CordId>,
    message: Message,
}

impl Packet {
    pub fn new(src: &Vcp, mesage: Message) -> Self {
        Packet {
            receiver: Receiver::Broadcast,
            sender_name: src.debug_name.clone(),
            sender_cid: src.c_id,
            message: mesage,
        }
    }
    pub fn new_unicast(src: &Vcp, dst: CordId, mesage: Message) -> Self {
        Packet {
            receiver: Receiver::Unicast(dst),
            sender_name: src.debug_name.clone(),
            sender_cid: src.c_id,
            message: mesage,
        }
    }

    pub fn is_for(&self, dst: Option<CordId>) -> bool {
        if let Receiver::Unicast(l) = self.receiver {
            if Some(l) != dst {
                // Unicast packet is not meant for this device
                return false;
            }
        }
        return true;
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
        let p_temp: CordId;
        for (&cid, neighbor) in self.neighbors.clone().iter() {
            if cid == S {
                if neighbor.successor.is_none() {
                    p_temp = E;
                } else if neighbor.successor == Some(E) {
                    p_temp = (S + E) / 2;
                } else {
                    p_temp = neighbor.successor.unwrap() - I * (neighbor.successor.unwrap() - cid);
                }
                self.c_id = Some(S);
                self.successor = Some(p_temp);
                self.send(&Packet::new_unicast(
                    self,
                    cid,
                    Message::SendUpdatePredecessor {
                        new_position: p_temp,
                    },
                ));
                break;
            } else if cid == E {
                if neighbor.successor == Some(S) {
                    p_temp = (S + E) / 2;
                } else {
                    p_temp =
                        neighbor.predecessor.unwrap() - I * (neighbor.predecessor.unwrap() - cid);
                }
                self.c_id = Some(E);
                self.predecessor = Some(p_temp);
                self.send(&Packet::new_unicast(
                    self,
                    cid,
                    Message::SendUpdateSuccessor {
                        new_position: p_temp,
                    },
                ));
                break;
            } else {
                todo!()
            }
        }
    }

    /// Method is called, when a new message is received.
    pub fn receive(&mut self, packet: &Packet) {
        if !packet.is_for(self.c_id) {
            return;
        }

        match packet.message {
            Message::Text(_) => todo!(),
            Message::Hello(neigh) => {
                let r = self.neighbors.insert(
                    packet.sender_cid.expect("Expected that CID is set"),
                    neigh.clone(),
                );
                if r.is_none() {
                    println!(
                        "{}: received '{:?}' from {}",
                        self.debug_name, packet.message, packet.sender_name
                    );
                }
            }
            Message::SendUpdatePredecessor { new_position } => {
                let old_cid = self.c_id.clone();
                self.c_id = Some(new_position);
                self.predecessor = packet.sender_cid;
                if old_cid != Some(new_position) {
                    if let Some(dst) = self.successor {
                        self.send(&Packet::new_unicast(
                            self,
                            dst,
                            Message::SendUpdatePredecessor { new_position: dst },
                        ));
                    }
                }
            }
            Message::SendUpdateSuccessor { new_position } => {
                let old_cid = self.c_id.clone();
                self.c_id = Some(new_position);
                self.successor = packet.sender_cid;
                if old_cid != Some(new_position) {
                    if let Some(dst) = self.predecessor {
                        self.send(&Packet::new_unicast(
                            self,
                            dst,
                            Message::SendUpdateSuccessor { new_position: dst },
                        ));
                    }
                }
            }
        }
    }

    fn send(&mut self, packet: &Packet) {
        println!("Sending {:?}", packet);
        self.outgoing_msgs.push(packet.clone());
    }

    /// Function that HAS to be called periodically
    pub fn timer_call(&mut self) {
        if self.c_id.is_none() {
            self.set_my_position();
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
}
