#![allow(unused_imports)]

use world::{GridWorld};

#[test]
fn world_gridworld_new() {
    let mut gw = GridWorld::new();

    assert!(gw.isItemOn(~"hp11").unwrap());
    assert!(gw.isItemOff(~"hp21").unwrap());
    assert!(gw.isItemOff(~"hp31").unwrap());

    gw.perform_action(~"handf");
    assert!(gw.isItemOn(~"hp12").unwrap());
    assert!(gw.isItemOff(~"hp21").unwrap());

    gw.perform_action(~"handf");
    assert!(gw.isItemOn(~"hp13").unwrap());
    assert!(gw.isItemOff(~"hp31").unwrap());

    gw.perform_action(~"handf");
    assert!(gw.isItemOn(~"hp13").unwrap());
    assert!(gw.isItemOff(~"hp31").unwrap());
}
