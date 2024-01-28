use std::fs;

use dummy::VirtManager;

mod dummy;
mod vcp;

fn main() {
    let mut mgr = VirtManager::new();
    mgr.add_device((10, 20)); // will try to send the Init message
    mgr.add_device((10, 25));
    mgr.add_device((10, 30));
    mgr.add_device((10, 40));

    for _ in 0..10 {
        mgr.handle_messages();
        println!("...");
    }

    let _ = fs::write("out.dot", mgr.generate_graph());
}
