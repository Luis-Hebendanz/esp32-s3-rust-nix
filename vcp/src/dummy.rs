use std::{
    fs::{self, remove_file},
    io::Error,
    path::Path,
    process::Command,
};

use crate::vcp::*;
use petgraph::{dot::Dot, graph::Graph, stable_graph::NodeIndex};

/// Implementation for Virtual VCP Device
pub struct VirtDevice {
    vcp: Vcp,
    position: (i32, i32),
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
    /// Generates a GraphViz Diagraph in .dot Filefromat
    pub fn generate_graph(&self) -> String {
        const SCALE: f64 = 3.0;
        /// Should Virtual Nodes be displayed in the graph or only included in the 'parrent' node.
        const DISPLAY_VIRTUAL_NODES: bool = false;
        let mut g = Graph::<String, String>::new();

        // All Nodes and its Virtual Nodes, stored with Index and the Virtual Device Information
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
            // add edges by searching the right NodeIndex of successor
            if let Some(a) = v.successor {
                if let Some((b, _, _)) =
                    devs_and_virtuals.iter().find(|(_, _, p)| p.c_id == Some(a))
                {
                    g.add_edge(NodeIndex::new(*i), NodeIndex::new(*b), String::from("s"));
                }
            }
            // same with predeccesor
            if let Some(a) = v.predecessor {
                if let Some((b, _, _)) =
                    devs_and_virtuals.iter().find(|(_, _, p)| p.c_id == Some(a))
                {
                    g.add_edge(NodeIndex::new(*i), NodeIndex::new(*b), String::from("p"));
                }
            }
        }

        // Custom Attribute Functions to set the 2D Coodinates in the graph
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
pub fn save_to_png(dot_g: &String, path: &Path) -> Result<(), Error> {
    let dotfile = format!(
        "{}/{}.{}",
        path.parent().unwrap().to_str().unwrap(),
        path.file_stem().unwrap().to_str().unwrap(),
        "dot"
    );
    println!("{}", dotfile);

    let _ = fs::write(dotfile.clone(), dot_g).expect("msg");
    let cmd = Command::new("dot")
        .arg("-Kfdp")
        .arg("-n")
        .arg("-Tpng")
        .arg(dotfile.clone())
        .args(["-o", path.to_str().unwrap()])
        .status()
        .expect("command failed");
    remove_file(dotfile)?;
    Ok(())
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
