use petgraph::{
    dot::{Config, Dot},
    graph::Graph,
    stable_graph::NodeIndex,
};
use std::{
    fmt::Write,
    fs::{self, remove_file},
    io::Error,
    path::Path,
    process::Command,
    sync::mpsc::Receiver,
};

use crate::{
    dummy::*,
    vcp::{Message, Packet, Vcp},
};

pub struct GraphViz {}

const SCALE: f64 = 3.0;
const DISPLAY_VIRTUAL_NODES: bool = false;
impl GraphViz {
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

    /// Get GraphViz Nodes for displaying the data that is stored in a node
    fn get_data_storage(virt: &VirtManager) -> String {
        let mut res = String::new();
        let mut i = 0;
        for dev in &virt.devices {
            let mut message_node = String::new();
            write!(
                &mut message_node,
                r#"
                data{i} -> {i}[style="rounded,dotted" dir=none]
                data{i} 
                [ 
                    shape=box, 
                    color=red
                    _pos = "{},{}!",
                    "#,
                dev.position.0 as f64 / SCALE,
                dev.position.1 as f64 / SCALE
            )
            .unwrap();
            i += 1;

            write!(&mut message_node, "label = \"Data: \n").unwrap();
            let mut count = 0;
            for m in &dev.vcp.data_storage {
                count += 1;
                write!(&mut message_node, "{}: {}\n", m.sender_cid, m.text,).unwrap();
            }
            write!(&mut message_node, "\"").unwrap();

            write!(&mut message_node, "]").unwrap();
            if count > 0 {
                res += &message_node;
            }
        }

        res
    }

    /// Get GraphViz Nodes for visualizing messages that are send
    fn get_data_messages(virt: &VirtManager) -> String {
        let mut res = String::new();
        let mut i = 0;
        for dev in &virt.devices {
            let mut message_node = String::new();
            write!(
                &mut message_node,
                r#"
                msg{i} -> {i} [style="rounded,dotted" dir=none]
                msg{i} 
                [ 
                    shape=box, 
                    color=blue
                    _pos = "{},{}!",
                    "#,
                dev.position.0 as f64 / SCALE,
                dev.position.1 as f64 / SCALE
            )
            .unwrap();
            i += 1;

            let mut outgoing: Vec<&Packet> = Vec::new();

            outgoing.extend(dev.vcp.outgoing_msgs.iter());
            for virt in &dev.vcp.virtual_nodes {
                if DISPLAY_VIRTUAL_NODES {}
                outgoing.extend(virt.outgoing_msgs.iter());
            }

            write!(&mut message_node, "label = \"").unwrap();
            let mut count = 0;
            for m in outgoing {
                match m.message {
                    Message::Text(ref s) => {
                        count += 1;
                        write!(
                            &mut message_node,
                            "[{} -> {}] {s}\n",
                            m.sender_cid.unwrap(),
                            m.final_cid.unwrap()
                        )
                        .unwrap();
                    }
                    _ => {}
                }
            }
            write!(&mut message_node, "\"").unwrap();

            write!(&mut message_node, "]").unwrap();
            if count > 0 {
                res += &message_node;
            }
        }

        res
    }
    /// Generates a GraphViz Diagraph in .dot Filefromat
    pub fn generate_graph(virt: &VirtManager) -> String {
        /// Should Virtual Nodes be displayed in the graph or only included in the 'parrent' node.
        let mut g = Graph::<String, String>::new();

        // All Nodes and its Virtual Nodes, stored with Index and the Virtual Device Information
        let mut devs_and_virtuals: Vec<(usize, &VirtDevice, &Vcp)> = Vec::new();

        let mut i = 0;
        for dev in &virt.devices {
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
            &[Config::GraphContentOnly],
            &get_edge,
            &get_node,
        );
        let mut extras = String::new();
        extras += &GraphViz::get_data_messages(&virt);
        extras += &GraphViz::get_data_storage(&virt);
        format!("digraph {{\n {} \n{extras}\n }}", dot_g.to_string())
    }
}
