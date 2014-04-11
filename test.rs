#![allow(unused_imports)]

use world::{GridWorld};

#[test]
fn world_gridworld_new() {
    let mut gw = GridWorld::new();
    assert!(gw.isItemOn(~"hp11").unwrap());

    gw.perform_action(~"handf");
    assert!(gw.isItemOn(~"hp12").unwrap());

    gw.perform_action(~"handf");
    assert!(gw.isItemOn(~"hp13").unwrap());

    gw.perform_action(~"handf");
    assert!(gw.isItemOn(~"hp13").unwrap());
}
