use std::collections::{BTreeMap, HashMap, HashSet};

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

type CordId = u32;

#[derive(Clone, Debug, Copy)]
pub struct NeighborInfo {
    predecessor: Option<CordId>,
    successor: Option<CordId>,
    age: u64,
}

pub struct Vcp {
    /// The Cord Id. `None` means not assigned. And 0 is the first device.
    pub c_id: Option<CordId>,
    pub debug_name: String,
    /// All messages that should be sent
    pub outgoing_msgs: Vec<Packet>,

    pub predecessor: Option<CordId>,
    pub successor: Option<CordId>,

    pub neighbors: BTreeMap<CordId, NeighborInfo>,

    ticks: u64,
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
            neighbors: BTreeMap::new(),
            ticks: 0,
        }
    }

    fn set_my_position(&mut self) {
        if self.neighbors.len() == 0 {
            return;
        }

        const S: CordId = 0;
        const E: CordId = 100;
        const I: f64 = 0.5;

        fn position(a: CordId, b: CordId) -> CordId {
            let tmp = a as f64 - I * (a as f64 - b as f64);

            return tmp as CordId;
        }

        for (&cid, neighbor) in self.neighbors.clone().iter() {
            let p_temp: CordId;
            if cid == S {
                // neigh is the first node
                if neighbor.successor.is_none() {
                    p_temp = E;
                } else if neighbor.successor == Some(E) {
                    p_temp = (S + E) / 2;
                } else {
                    p_temp = position(neighbor.successor.unwrap(), cid);
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
                return;
            } else if cid == E {
                // neigh is the last node
                if neighbor.successor == Some(S) {
                    p_temp = (S + E) / 2;
                } else {
                    p_temp = position(neighbor.predecessor.unwrap(), cid);
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
                return;
            }
        }
        for (&cid, neighbor) in self.neighbors.clone().iter() {
            let p_temp: CordId;
            for (&cid2, _) in self.neighbors.clone().iter() {
                if cid2 == cid {
                    continue;
                }
                if neighbor.predecessor == Some(cid2) {
                    p_temp = (cid + cid2) / 2;
                    self.c_id = Some(p_temp);
                    self.predecessor = Some(cid);
                    self.successor = Some(cid2);
                    self.send(&Packet::new_unicast(
                        self,
                        cid,
                        Message::SendUpdateSuccessor { new_position: cid },
                    ));
                    self.send(&Packet::new_unicast(
                        self,
                        cid2,
                        Message::SendUpdatePredecessor { new_position: cid2 },
                    ));
                    return;
                }
            }
        }
        println!("virtual");
        //todo!("virtual");
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
                    neigh.clone(), // age is set to 0
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

                // TODO is the recursive approach right?
                /*if old_cid != Some(new_position) {
                    if let Some(dst) = self.successor {
                        self.send(&Packet::new_unicast(
                            self,
                            dst,
                            Message::SendUpdatePredecessor { new_position: dst },
                        ));
                    }
                }*/
            }
            Message::SendUpdateSuccessor { new_position } => {
                let old_cid = self.c_id.clone();
                self.c_id = Some(new_position);
                self.successor = packet.sender_cid;
                /*
                if old_cid != Some(new_position) {
                    if let Some(dst) = self.predecessor {
                        self.send(&Packet::new_unicast(
                            self,
                            dst,
                            Message::SendUpdateSuccessor { new_position: dst },
                        ));
                    }
                }*/
            }
        }
    }

    fn send(&mut self, packet: &Packet) {
        //println!("Sending {:?}", packet);
        self.outgoing_msgs.push(packet.clone());
    }

    /// Function that HAS to be called periodically
    pub fn timer_call(&mut self) {
        self.ticks += 1;
        if self.c_id.is_none() {
            if (self.ticks > 1) {
                self.set_my_position();
            }
        } else {
            self.send(&Packet::new(
                &self,
                Message::Hello(NeighborInfo {
                    predecessor: self.predecessor,
                    successor: self.successor,
                    age: 0,
                }),
            ));
        }

        // change age
        for n in self.neighbors.iter_mut() {
            n.1.age += 1;
        }
        // remove all older than 5
        self.neighbors.retain(|_, n| n.age < 5);

        if let Some(cid) = self.c_id {
            let mut max = None;
            let mut min = None;
            for (&n, _) in self.neighbors.iter() {
                if n > cid {
                    if let Some(max1) = max {
                        if max1 < n {
                            continue;
                        }
                    }
                    max = Some(n);
                }
                if n < cid {
                    if let Some(min1) = min {
                        if min1 > n {
                            continue;
                        }
                    }
                    min = Some(n);
                }
            }
            self.successor = max;
            self.predecessor = min;
        }
    }
}
