use crate::playground::Playground;
use rand::Rng;
use std::fs;

mod dummy;
mod graphing;
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

    play.send_text_data(0, 1000, String::from("Hello, Falko"));
    play.send_text_data(1, 999, String::from("2 Hello, Sascha"));
    play.send_text_data(999, 1, String::from("3 Hello, Sigrid"));
    play.send_text_data(510, 563, String::from("4 Hello, Joana"));
    play.send_text_data(0, 625, String::from("5 Hello, All"));
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
    assert!(play.mgr.find_inconsitency().is_none());
}
