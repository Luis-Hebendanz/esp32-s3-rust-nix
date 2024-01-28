use crate::vcp::*;
use petgraph::{dot::Dot, graph::Graph, stable_graph::NodeIndex};
/// Implementation for Virtual VCP Device
pub struct VirtDevice {
    vcp: Vcp,
    position: (i32, i32),
}

impl Communication for VirtDevice {
    fn broadcast(&mut self, p: &Packet) {
        // The virtual sending is handled in VirtManager
        //self.vcp.outgoing_msgs.push(p.clone())
    }
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
    devices: Vec<VirtDevice>,

    // The max sending distance
    range: i32,
}

impl VirtManager {
    pub fn handle_messages(&mut self) {
        // send all message that are in outgoing_msgs to all devices
        let mut sends: Vec<(usize, Packet, usize)> = Vec::new();
        for (s, ss) in self.devices.iter().enumerate() {
            for m in &ss.vcp.outgoing_msgs {
                for (r, rr) in self.devices.iter().enumerate() {
                    if s == r {
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

        for (s, m, r) in sends {
            self.devices[r].vcp.receive(&m);
        }
    }

    pub fn add_device(&mut self, pos: (i32, i32)) {
        let mut d = VirtDevice::new(self.devices.len() == 0);
        d.position = pos;
        d.vcp.debug_name = format!("Dev: {}", self.devices.len());
        if self.devices.len() == 0 {
            d.vcp.send_init_message();
        }
        self.devices.push(d);
    }

    pub fn new() -> Self {
        VirtManager {
            devices: Vec::new(),
            range: 10,
        }
    }
    /// Generates a GraphViz Diagraph in .dot Filefromat
    pub fn generate_graph(&self) -> String {
        let mut g = Graph::<&String, String>::new();

        for (i, d) in self.devices.iter().enumerate() {
            g.add_node(&d.vcp.debug_name);

            if let Some(a) = d.vcp.successor {
                if let Some(b) = self.devices.iter().position(|p| p.vcp.c_id == Some(a)) {
                    g.add_edge(NodeIndex::new(i), NodeIndex::new(b), String::from(""));
                }
            }
            if let Some(a) = d.vcp.predecessor {
                if let Some(b) = self.devices.iter().position(|p| p.vcp.c_id == Some(a)) {
                    g.add_edge(NodeIndex::new(b), NodeIndex::new(i), String::from(""));
                }
            }
        }
        let scale = 10;
        let get_edge = |a, b| String::from("");
        let get_node = |a, b: (NodeIndex, &&String)| {
            if let Some(d) = self.devices.get(b.0.index()) {
                format!(
                    "pos = \"{},{}!\"",
                    d.position.0 / scale,
                    d.position.1 / scale
                )
            } else {
                String::from("\npos = \"0,0!\"")
            }
        };

        let dot_g = Dot::with_attr_getters(
            //Dot::with_config(
            &g,
            &[],
            &get_edge,
            &get_node,
        );

        dot_g.to_string()
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
}
