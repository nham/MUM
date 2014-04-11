#![allow(unused_imports)]

use world::{GridWorld};

#[test]
fn world_gridworld_new() {
    let mut gw = GridWorld::new();
    gw.perform_action(~"handf");
    gw.perform_action(~"handf");
    gw.perform_action(~"handf");
    gw.perform_action(~"handf");
}
