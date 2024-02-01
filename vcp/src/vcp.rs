use std::{collections::BTreeMap, fmt};

pub trait Communication {
    fn broadcast(&mut self, p: &Packet);
}

#[derive(Clone, Debug)]
pub enum Message {
    Hello(NeighborInfo),
    SendUpdatePredecessor { new_position: CordId },
    SendUpdateSuccessor { new_position: CordId },
    CreateVirtualNode { virtual_position: CordId },
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

    /// check if self is the receiver of dst. Or if dst in broadcast
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
type NeighborMap = BTreeMap<CordId, NeighborInfo>;
#[derive(Clone, Debug, Copy)]
/// Packs information about all neighbors, that have to be remembered
pub struct NeighborInfo {
    predecessor: Option<CordId>,
    successor: Option<CordId>,
    is_virtual: bool,
    age: u64,
}

pub struct Vcp {
    /// The Cord Id (the Position). `None` means not assigned. And 0 is the first device.
    pub c_id: Option<CordId>,
    pub debug_name: String,
    /// All messages that should be sent
    pub outgoing_msgs: Vec<Packet>,

    pub predecessor: Option<CordId>,
    pub successor: Option<CordId>,

    pub neighbors: NeighborMap,

    ticks: u64,
    pub virtual_nodes: Vec<Vcp>,
    is_virtual: bool,
}

impl fmt::Display for Vcp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prev = self
            .predecessor
            .map(|a| a.to_string())
            .unwrap_or("?".into());
        let succ = self.successor.map(|a| a.to_string()).unwrap_or("?".into());
        if self.is_virtual {
            write!(f, "v")?;
        }
        if let Some(cid) = self.c_id {
            write!(f, "{}", cid)?;
        } else {
        }
        write!(f, "'{}'", self.debug_name)?;
        if !self.virtual_nodes.is_empty() {
            write!(f, "\nvirt{{")?;
            for virt in &self.virtual_nodes {
                write!(f, "{},", virt.c_id.unwrap())?;
            }
            write!(f, "}}")?;
        }

        write!(f, ":\np{} s{}", prev, succ)?;
        Ok(())
    }
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
            ticks: 0, // the clock of the node
            virtual_nodes: Vec::new(),
            is_virtual: false,
        }
    }

    /// Function to request a CID, and send PositionChange Requests to Other nodes
    fn set_my_position(&mut self) {
        if self.neighbors.len() == 0 {
            return;
        }

        const S: CordId = 0;
        const E: CordId = 1000;
        const I: f64 = 0.5;

        fn position(a: CordId, b: CordId) -> CordId {
            let tmp = a as f64 - I * (a as f64 - b as f64);

            return tmp as CordId;
        }

        for (&cid, neighbor) in self.neighbors.clone().iter() {
            if cid == S {
                // neigh is the first node
                let new_position: CordId;
                if neighbor.successor.is_none() {
                    new_position = E;
                } else if neighbor.successor == Some(E) {
                    new_position = (S + E) / 2;
                } else {
                    new_position = position(neighbor.successor.unwrap(), cid);
                }
                self.c_id = Some(S);
                self.successor = Some(new_position);
                self.send(&Packet::new_unicast(
                    self,
                    cid,
                    Message::SendUpdatePredecessor { new_position },
                ));
                return;
            } else if cid == E {
                // neigh is the last node
                let new_position: CordId;
                if neighbor.successor == Some(S) {
                    new_position = (S + E) / 2;
                } else {
                    new_position = position(neighbor.predecessor.unwrap(), cid);
                }
                self.c_id = Some(E);
                self.predecessor = Some(new_position);
                self.send(&Packet::new_unicast(
                    self,
                    cid,
                    Message::SendUpdateSuccessor { new_position },
                ));
                return;
            }
        }

        // ... no neighbor at End or Start found

        // search for two neighbors which are direct neighbors
        // cid -> cid2 and request to put in between:
        // cid -> self.c_id -> cid2
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

        // Otherwise request to create a virtual node
        if let Some((&cid, neigh)) = self.neighbors.iter().find(|n| !n.1.is_virtual) {
            // find a neighbor which is not virtual
            let new_virt = (cid + neigh.successor.unwrap()) / 2;
            let new_cid = (cid + new_virt) / 2;
            self.c_id = Some(new_cid);
            self.predecessor = Some(cid);
            self.successor = Some(new_virt);
            self.send(&Packet::new_unicast(
                self,
                cid,
                Message::CreateVirtualNode {
                    virtual_position: new_virt,
                },
            ));
        }
    }

    /// Method is called, when a new message is received.
    pub fn receive(&mut self, packet: &Packet) {
        // call receive for all Sub Nodes
        for virt in self.virtual_nodes.iter_mut() {
            virt.receive(packet);
        }
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
                let _old_cid = self.c_id.clone();
                self.c_id = Some(new_position);
                self.predecessor = packet.sender_cid;
            }
            Message::SendUpdateSuccessor { new_position } => {
                let _old_cid = self.c_id.clone();
                self.c_id = Some(new_position);
                self.successor = packet.sender_cid;
            }
            Message::CreateVirtualNode { virtual_position } => {
                let mut new_vcp = Vcp::new(false);
                new_vcp.c_id = Some(virtual_position);
                new_vcp.debug_name = format!("Virt {}", self.debug_name);
                new_vcp.is_virtual = true;
                self.virtual_nodes.push(new_vcp);
            }
        }
    }

    fn send(&mut self, packet: &Packet) {
        self.outgoing_msgs.push(packet.clone());
    }

    /// Function that HAS to be called periodically
    pub fn timer_call(&mut self) {
        self.ticks += 1;
        if self.c_id.is_none() {
            // request own position
            if self.ticks > 1 {
                self.set_my_position();
            }
        } else {
            // send hello messages, regularly
            self.send(&Packet::new(
                &self,
                Message::Hello(NeighborInfo {
                    predecessor: self.predecessor,
                    successor: self.successor,
                    is_virtual: self.is_virtual,
                    age: 0,
                }),
            ));
        }

        self.update_neighbor_ages();

        // find best successor and predecessor
        let (s, p) = Vcp::calc_successor_predecessor(&self);
        self.successor = s;
        self.predecessor = p;

        // Call timer of all virtual_nodes
        for virt in self.virtual_nodes.iter_mut() {
            virt.timer_call();
            self.outgoing_msgs.append(&mut virt.outgoing_msgs); // and move outgoing messages to self
        }
    }

    /// Change age of all neighbor information.
    /// This useful if information about neighbors get outdated
    /// Also delete neighbors that are too old.
    fn update_neighbor_ages(&mut self) {
        for n in self.neighbors.iter_mut() {
            n.1.age += 1;
        }
        // remove all older than 5
        self.neighbors.retain(|_, n| n.age < 5);
    }

    /// Calculate the predecessor and successor by choosing the closest neighbor.
    fn calc_successor_predecessor(&self) -> (Option<u32>, Option<u32>) {
        let mut succ = None;
        let mut pred = None;

        fn set_if_smaller(set: &Option<CordId>, new: CordId) -> Option<CordId> {
            if let Some(min1) = *set {
                if min1 > new {
                    return *set;
                }
            }
            return Some(new);
        }
        fn set_if_larger(set: &Option<CordId>, new: CordId) -> Option<CordId> {
            if let Some(min1) = *set {
                if min1 < new {
                    return *set;
                }
            }
            return Some(new);
        }

        if let Some(cid) = self.c_id {
            for (&n, _neigh) in self.neighbors.iter() {
                if n > cid {
                    succ = set_if_larger(&succ, n);
                }
                if n < cid {
                    pred = set_if_smaller(&pred, n);
                }
            }
        }
        (succ, pred)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_successor_predecessor() {
        let mut slf = Vcp::new(false);
        slf.c_id = Some(50);

        let ni = NeighborInfo {
            predecessor: None,
            successor: None,
            is_virtual: false,
            age: 0,
        };
        slf.neighbors.insert(60, ni);
        slf.neighbors.insert(70, ni);
        slf.neighbors.insert(55, ni);
        slf.neighbors.insert(45, ni);
        slf.neighbors.insert(10, ni);
        let (s, p) = Vcp::calc_successor_predecessor(&slf);

        assert_eq!(s, Some(55));
        assert_eq!(p, Some(45));
    }

    #[test]
    fn calc_successor_predecessor_virtual() {
        let mut slf = Vcp::new(false);
        slf.c_id = Some(50);

        let ni = NeighborInfo {
            predecessor: None,
            successor: None,
            is_virtual: true,
            age: 0,
        };
        slf.neighbors.insert(0, ni);
        slf.neighbors.insert(60, ni);
        slf.neighbors.insert(100, ni);
        let (s, p) = Vcp::calc_successor_predecessor(&slf);

        assert_eq!(s, Some(60));
        assert_eq!(p, Some(0));
    }
}
