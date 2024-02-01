use std::{fs, path::Path};

use dummy::VirtManager;

use crate::dummy::save_to_png;

mod dummy;
mod vcp;

fn main() {
    let mgr = VirtManager::new();

    let mut mgr = VirtManager::new();
    let mut age = 0;
    let mut old_graph: Option<String> = None;
    let mut ticks = |n: i32, mgr: &mut VirtManager| {
        for _ in 0..n {
            mgr.handle_messages();
            age += 1;
            let err = mgr.find_inconsitency();
            if err.is_some() {
                println!("Inconsisten at {age} {}", err.unwrap());
            }

            let name = format!("out/{:0>3}.png", age);
            let gr = mgr.generate_graph();
            if Some(&gr) != old_graph.as_ref() {
                save_to_png(&gr, Path::new(&name));
            }
            old_graph = Some(gr);
        }
    };
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
    let _ = fs::write("out.dot", mgr.generate_graph());
    assert!(mgr.find_inconsitency().is_none());
}
