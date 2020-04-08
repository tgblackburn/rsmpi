#![deny(warnings)]
#![allow(clippy::float_cmp)]
extern crate mpi;

use mpi::traits::*;
use std::{thread, time};

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();

    let x = std::f32::consts::PI;
    let mut y: f32 = 0.0;

    if world.rank() == 0 {
        mpi::request::scope(|scope| {
            let mut requests = Vec::new();
            for i in 1..world.size() {
                requests.push(
                    world
                        .process_at_rank(i)
                        .immediate_synchronous_send(scope, &x),
                );
            }

            println!("World size {}", world.size());
            while !requests.is_empty() {
                if let Some((index, _status)) = mpi::request::test_any(&mut requests) {
                    println!("Request with index {} completed", index);
                }
            }
        });
        println!("All requests completed");
    } else {
        thread::sleep(time::Duration::from_millis(
            (400 * ((world.rank() % 2) + 1)) as u64,
        ));

        world.any_process().receive_into(&mut y);
        println!("Process {} received data", world.rank());
    }
}
