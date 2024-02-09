use std::{fs, path::Path};

use crate::playground::Playground;
use dummy::VirtManager;
use rand::Rng;

mod dummy;
mod playground;
mod vcp;

fn example_fail(play: &mut Playground) {
    play.add_device(0, 0); // will try to send the Init message
    play.add_device(0, 5);
    play.add_device(0, 10);
    play.add_device(15, 0);
    play.add_device(15, 5);
    play.add_device(7, 5);
    play.ticks(10);
}
fn example1(play: &mut Playground) {
    play.add_device(2, -2); // will try to send the Init message
    play.add_device(3, 5);
    play.add_device(0, 14);
    play.add_device(4, 20);

    play.add_device(3, -2);
    play.add_device(10, 8);
    play.add_device(10, 4);
    play.add_device(-7, 5);
    play.mgr.devices.remove(0);
    play.ticks(10);
}

fn example1_send_data(play: &mut Playground) {
    play.add_device(2, -2); // will try to send the Init message
    play.add_device(3, 5);
    play.add_device(0, 14);
    play.add_device(4, 20);

    play.add_device(3, -2);
    play.add_device(10, 8);
    play.add_device(10, 4);
    play.add_device(-7, 5);
    play.mgr.devices.remove(0);
    play.ticks(10);

    let from: u32 = 0;
    let to: u32 = 1000;
    let text: String = String::from("Hello, me");
    play.send_text_data(from, to, text); //todo
    play.ticks(10);
}

fn example_rnd(play: &mut Playground) {
    let mut r = rand::thread_rng();
    for _ in 0..10 {
        play.add_device(r.gen_range(0..15), r.gen_range(0..15));
    }
    play.ticks(10);
}

fn main() {
    let mut play = Playground::new();
    //example_fail(&mut play);
    //example_rnd(&mut play);
    //example1(&mut play);
    example1_send_data(&mut play);
    let _ = fs::write("out.dot", play.mgr.generate_graph());
    assert!(play.mgr.find_inconsitency().is_none());
}
