use crate::vcp::*;
use petgraph::{dot::Dot, graph::Graph, stable_graph::NodeIndex};
/// Implementation for Virtual VCP Device
pub struct VirtDevice {
    vcp: Vcp,
    position: (i32, i32),
}

impl Communication for VirtDevice {
    fn broadcast(&mut self, _p: &Packet) {
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

    pub fn new() -> Self {
        VirtManager {
            devices: Vec::new(),
            range: 10,
        }
    }
    /// Generates a GraphViz Diagraph in .dot Filefromat
    pub fn generate_graph(&self) -> String {
        const SCALE: f64 = 3.0;
        const DISPLAY_VIRTUAL_NODES: bool = false;
        let mut g = Graph::<String, String>::new();

        let mut devs_and_virtuals: Vec<(usize, &VirtDevice, &Vcp)> = Vec::new();

        let mut i = 0;
        for dev in &self.devices {
            g.add_node(format!("{}", dev.vcp));
            devs_and_virtuals.push((i, dev, &dev.vcp));

            for virt in &dev.vcp.virtual_nodes {
                if DISPLAY_VIRTUAL_NODES {
                    i += 1;
                    g.add_node(format!("{}", virt));
                }
                devs_and_virtuals.push((i, dev, virt));
            }
            i += 1;
        }

        for (i, _, v) in &devs_and_virtuals {
            // add edges
            if let Some(a) = v.successor {
                if let Some((b, _, _)) =
                    devs_and_virtuals.iter().find(|(_, _, p)| p.c_id == Some(a))
                {
                    g.add_edge(NodeIndex::new(*i), NodeIndex::new(*b), String::from("s"));
                }
            }
            if let Some(a) = v.predecessor {
                if let Some((b, _, _)) =
                    devs_and_virtuals.iter().find(|(_, _, p)| p.c_id == Some(a))
                {
                    g.add_edge(NodeIndex::new(*i), NodeIndex::new(*b), String::from("p"));
                }
            }
        }
        let get_edge = |_, _b: petgraph::graph::EdgeReference<'_, String, _>| String::from(""); // b.weight().clone();
        let get_node = |_, b: (NodeIndex, &String)| {
            if let Some((_, d, _v)) = devs_and_virtuals.iter().find(|(i, _, _)| *i == b.0.index()) {
                format!(
                    "pos = \"{},{}!\"",
                    d.position.0 as f64 / SCALE,
                    d.position.1 as f64 / SCALE
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

    #[test]
    fn unicast() {
        let dev1 = VirtDevice::new(true);
        let dev2 = VirtDevice::new(false);
    }
    #[test]
    fn failing() {
        let mut mgr = VirtManager::new();
        mgr.add_device((0, 0)); // will try to send the Init message
        mgr.add_device((3, 5));
        mgr.handle_messages();
        mgr.handle_messages();
        mgr.add_device((0, 12));
        mgr.add_device((4, 20));

        for _ in 0..10 {
            mgr.handle_messages();
            println!("...");
        }
        mgr.add_device((0, -4));
        mgr.handle_messages();
        mgr.handle_messages();
        mgr.handle_messages();

        mgr.add_device((0, 8));
        mgr.handle_messages();
        mgr.handle_messages();
        mgr.handle_messages();
        mgr.handle_messages();
    }
}
