use std::fs;

use dummy::VirtManager;

mod dummy;
mod vcp;

fn main() {
    let mut mgr = VirtManager::new();
    mgr.add_device((0, 0)); // will try to send the Init message
    mgr.add_device((3, 5));
    mgr.handle_messages();
    mgr.handle_messages();
    mgr.add_device((0, 12));
    //mgr.add_device((10, 40));

    for _ in 0..10 {
        mgr.handle_messages();
        println!("...");
    }
    //mgr.add_device((0, -4));
    //mgr.handle_messages();

    let _ = fs::write("out.dot", mgr.generate_graph());
}
