use crate::vcp::*;

/// Implementation for Virtual VCP Device
pub struct VirtDevice {
    pub vcp: Vcp,
    pub position: (i32, i32),
}

impl VirtDevice {
    pub fn new(is_first: bool) -> Self {
        let v = VirtDevice {
            vcp: Vcp::new(is_first),
            position: (0, 0),
        };
        v
    }
}

/// VirtManager contains all devices and message the "sending" of messages.
/// It checks if the "devices" can hear each other by checking the distance.
pub struct VirtManager {
    pub devices: Vec<VirtDevice>,

    // The max sending distance
    range: i32,
}

impl VirtManager {
    pub fn handle_messages(&mut self) {
        // send all message that are in outgoing_msgs to all devices
        let mut sends: Vec<(usize, Packet, usize)> = Vec::new();
        for (s, ss) in self.devices.iter().enumerate() {
            for m in &ss.vcp.outgoing_msgs {
                //check if receiver is virtual
                if m.is_type_data() {
                    for (index_real_node, real_node) in self.devices.iter().enumerate() {
                        //go through all nodes. If receiver is found in virtuel nodes of one node, replace receiver with real node
                        let ref virt_nodes_list = real_node.vcp.virtual_nodes;
                        //search through virtual nodes in list
                        for virt_node in virt_nodes_list.iter() {
                            //is package for one of the virtual nodes?
                            if m.is_for(virt_node.c_id) {
                                //set real node as receiver instead of its virtual node by replacing packet with new one
                                let real_node_cid = real_node
                                    .vcp
                                    .c_id
                                    .expect("Expected receiving node to have cid");
                                let virt_node_cid =
                                    virt_node.c_id.expect("Expected receiving node to have cid");
                                let r = m.new_receiver(real_node_cid);
                                sends.push((s, r.clone(), index_real_node));
                                println!("Receiver node {} is virtual. Sending to its real node {} instead.", virt_node_cid, real_node_cid);
                            }
                        }
                    }
                }

                for (r, rr) in self.devices.iter().enumerate() {
                    if s == r {
                        continue;
                    }

                    if !m.is_for(rr.vcp.c_id) {
                        continue;
                    }

                    let dist_sqr = ((ss.position.0 - rr.position.0).pow(2)
                        + (ss.position.1 - rr.position.1).pow(2))
                        as f64;

                    if dist_sqr > self.range.pow(2).into() {
                        continue;
                    }
                    sends.push((s, m.clone(), r));
                }
            }
        }
        for d in &mut self.devices {
            d.vcp.outgoing_msgs.clear();
        }

        for (_s, m, r) in sends {
            self.devices[r].vcp.receive(&m);
        }

        // send hello message

        for d in &mut self.devices {
            d.vcp.timer_call();
        }
    }

    pub fn add_device(&mut self, pos: (i32, i32)) {
        let mut d = VirtDevice::new(self.devices.len() == 0);
        d.position = pos;
        d.vcp.debug_name = format!("Dev: {}", self.devices.len());
        if self.devices.len() == 0 {
            d.vcp.c_id = Some(0);
        }
        d.vcp.timer_call();
        self.devices.push(d);
    }

    pub fn send_text_data(&mut self, from: u32, to: u32, text: String) {
        //find start node that is closed to the "from" id

        let mut best_sender_index: Option<usize> = None;
        let mut smallest_diff = 1000;

        for (s, ss) in self.devices.iter().enumerate() {
            if let Some(cid) = ss.vcp.c_id {
                let diff = cid.abs_diff(from);
                if diff < smallest_diff {
                    smallest_diff = diff;
                    //if this node's cid is close to the "from", remember its index
                    best_sender_index = Some(s);
                }
            }
        }
        //check if we found a sender and instruct the node to send
        let index = best_sender_index.expect("Connot send, bc no sender exist");
        self.devices[index].vcp.send_text_data(to, text);
    }

    pub fn new() -> Self {
        VirtManager {
            devices: Vec::new(),
            range: 10,
        }
    }

    pub fn find_inconsitency(&self) -> Option<String> {
        let mut devs_and_virtuals: Vec<(&VirtDevice, &Vcp)> = Vec::new();

        for dev in &self.devices {
            devs_and_virtuals.push((dev, &dev.vcp));

            for virt in &dev.vcp.virtual_nodes {
                devs_and_virtuals.push((dev, virt));
            }
        }
        if devs_and_virtuals
            .iter()
            .find(|(_, a)| a.c_id.is_none())
            .is_some()
        {
            return Some(format!("Some nodes don't have a valid cid"));
        }

        // check if each CID is unique
        let mut ids: Vec<_> = devs_and_virtuals
            .iter()
            .map(|(_, a)| a.c_id.unwrap())
            .collect();
        ids.sort();
        ids.dedup();

        if ids.len() != devs_and_virtuals.len() {
            return Some(format!("Some cids are not unique"));
        }

        // check if all successor and predecessor exist
        for (_, v) in &devs_and_virtuals {
            if let Some(a) = v.successor {
                if devs_and_virtuals
                    .iter()
                    .find(|(_, p)| p.c_id == Some(a))
                    .is_none()
                {
                    return Some(format!("{} successor does not exist", v.c_id.unwrap()));
                }
            }
            if let Some(a) = v.predecessor {
                if devs_and_virtuals
                    .iter()
                    .find(|(_, p)| p.c_id == Some(a))
                    .is_none()
                {
                    return Some(format!("{} predecessor does not exist", v.c_id.unwrap()));
                }
            }
        }

        // check there is only one End and one Start
        if devs_and_virtuals
            .iter()
            .filter(|(_, p)| p.successor.is_none())
            .count()
            != 1
        {
            return Some(format!("Too many loose ends"));
        }
        if devs_and_virtuals
            .iter()
            .filter(|(_, p)| p.predecessor.is_none())
            .count()
            != 1
        {
            return Some(format!("Too many loose ends"));
        }

        return None;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works2() {
        let dev1 = VirtDevice::new(true);
        let dev2 = VirtDevice::new(false);
    }

    #[test]
    fn unicast() {
        let dev1 = VirtDevice::new(true);
        let dev2 = VirtDevice::new(false);
    }

    #[test]
    fn complex_example() {
        let mut mgr = VirtManager::new();
        fn ticks(n: i32, mgr: &mut VirtManager) {
            for _ in 0..n {
                mgr.handle_messages();
            }
        }
        mgr.add_device((2, -2)); // will try to send the Init message
        ticks(10, &mut mgr);
        mgr.add_device((3, 5));
        ticks(10, &mut mgr);
        mgr.add_device((0, 14));
        ticks(10, &mut mgr);
        mgr.add_device((4, 20));

        ticks(10, &mut mgr);
        mgr.add_device((3, -8));
        ticks(20, &mut mgr);
        mgr.add_device((10, 8));
        ticks(30, &mut mgr);
        mgr.add_device((10, 4));
        ticks(30, &mut mgr);
        mgr.add_device((-7, 5));
        ticks(30, &mut mgr);
        assert!(mgr.find_inconsitency().is_none());
    }
}
