use std::fs;

use dummy::VirtManager;

mod dummy;
mod vcp;

fn main() {
    let mut mgr = VirtManager::new();

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
    let _ = fs::write("out.dot", mgr.generate_graph());
}
