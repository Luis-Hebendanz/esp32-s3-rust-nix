use dummy::VirtManager;

mod dummy;
mod vcp;

fn main() {
    let mut mgr = VirtManager::new();
    mgr.add_device((10, 20));
    mgr.add_device((10, 25));
    mgr.add_device((10, 30));

    mgr.handle_messages();
}
