use crate::dummy::{save_to_png, VirtManager};
use std::path::Path;

pub struct Playground {
    pub mgr: VirtManager,
    pub age: u32,
    old_graph: Option<String>,
}
impl Playground {
    pub fn ticks(&mut self, n: u8) {
        for _ in 0..n {
            self.mgr.handle_messages();
            self.age += 1;
            let err = self.mgr.find_inconsitency();
            if err.is_some() {
                println!("Inconsisten at {} {}", self.age, err.unwrap());
            }

            let name = format!("out/{:0>3}.png", self.age);
            let gr = self.mgr.generate_graph();
            if Some(&gr) != self.old_graph.as_ref() {
                save_to_png(&gr, Path::new(&name));
            }
            self.old_graph = Some(gr);
        }
    }

    pub fn add_device(&mut self, x: i32, y: i32) {
        self.mgr.add_device((x, y));
        self.ticks(10);
    }

    pub fn new() -> Playground {
        Playground {
            mgr: VirtManager::new(),
            age: 0,
            old_graph: None,
        }
    }
}