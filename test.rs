#![allow(unused_imports)]

use world::{GridWorld};

#[test]
fn world_gridworld_perform_action_hand() {
    let mut gw = GridWorld::new();

    // f, f, f, r, r, r, b, b, b, l, l, l

    assert!(
            gw.isItemOn(~"hp11").unwrap()
         && gw.isItemOff(~"hp12").unwrap()
         && gw.isItemOff(~"hp13").unwrap()
         && gw.isItemOff(~"hp21").unwrap()
         && gw.isItemOff(~"hp22").unwrap()
         && gw.isItemOff(~"hp23").unwrap()
         && gw.isItemOff(~"hp31").unwrap()
         && gw.isItemOff(~"hp32").unwrap()
         && gw.isItemOff(~"hp33").unwrap()
    );

    gw.perform_action(~"handf");

    assert!(
            gw.isItemOff(~"hp11").unwrap()
         && gw.isItemOn(~"hp12").unwrap()
         && gw.isItemOff(~"hp13").unwrap()
         && gw.isItemOff(~"hp21").unwrap()
         && gw.isItemOff(~"hp22").unwrap()
         && gw.isItemOff(~"hp23").unwrap()
         && gw.isItemOff(~"hp31").unwrap()
         && gw.isItemOff(~"hp32").unwrap()
         && gw.isItemOff(~"hp33").unwrap()
    );


    gw.perform_action(~"handf");

    assert!(
            gw.isItemOff(~"hp11").unwrap()
         && gw.isItemOff(~"hp12").unwrap()
         && gw.isItemOn(~"hp13").unwrap()
         && gw.isItemOff(~"hp21").unwrap()
         && gw.isItemOff(~"hp22").unwrap()
         && gw.isItemOff(~"hp23").unwrap()
         && gw.isItemOff(~"hp31").unwrap()
         && gw.isItemOff(~"hp32").unwrap()
         && gw.isItemOff(~"hp33").unwrap()
    );

    gw.perform_action(~"handf");

    assert!(
            gw.isItemOff(~"hp11").unwrap()
         && gw.isItemOff(~"hp12").unwrap()
         && gw.isItemOn(~"hp13").unwrap()
         && gw.isItemOff(~"hp21").unwrap()
         && gw.isItemOff(~"hp22").unwrap()
         && gw.isItemOff(~"hp23").unwrap()
         && gw.isItemOff(~"hp31").unwrap()
         && gw.isItemOff(~"hp32").unwrap()
         && gw.isItemOff(~"hp33").unwrap()
    );


    gw.perform_action(~"handr");

    assert!(
            gw.isItemOff(~"hp11").unwrap()
         && gw.isItemOff(~"hp12").unwrap()
         && gw.isItemOff(~"hp13").unwrap()
         && gw.isItemOff(~"hp21").unwrap()
         && gw.isItemOff(~"hp22").unwrap()
         && gw.isItemOn(~"hp23").unwrap()
         && gw.isItemOff(~"hp31").unwrap()
         && gw.isItemOff(~"hp32").unwrap()
         && gw.isItemOff(~"hp33").unwrap()
    );

    gw.perform_action(~"handr");

    assert!(
            gw.isItemOff(~"hp11").unwrap()
         && gw.isItemOff(~"hp12").unwrap()
         && gw.isItemOff(~"hp13").unwrap()
         && gw.isItemOff(~"hp21").unwrap()
         && gw.isItemOff(~"hp22").unwrap()
         && gw.isItemOff(~"hp23").unwrap()
         && gw.isItemOff(~"hp31").unwrap()
         && gw.isItemOff(~"hp32").unwrap()
         && gw.isItemOn(~"hp33").unwrap()
    );

    gw.perform_action(~"handr");

    assert!(
            gw.isItemOff(~"hp11").unwrap()
         && gw.isItemOff(~"hp12").unwrap()
         && gw.isItemOff(~"hp13").unwrap()
         && gw.isItemOff(~"hp21").unwrap()
         && gw.isItemOff(~"hp22").unwrap()
         && gw.isItemOff(~"hp23").unwrap()
         && gw.isItemOff(~"hp31").unwrap()
         && gw.isItemOff(~"hp32").unwrap()
         && gw.isItemOn(~"hp33").unwrap()
    );

    gw.perform_action(~"handb");

    assert!(
            gw.isItemOff(~"hp11").unwrap()
         && gw.isItemOff(~"hp12").unwrap()
         && gw.isItemOff(~"hp13").unwrap()
         && gw.isItemOff(~"hp21").unwrap()
         && gw.isItemOff(~"hp22").unwrap()
         && gw.isItemOff(~"hp23").unwrap()
         && gw.isItemOff(~"hp31").unwrap()
         && gw.isItemOn(~"hp32").unwrap()
         && gw.isItemOff(~"hp33").unwrap()
    );

    gw.perform_action(~"handb");

    assert!(
            gw.isItemOff(~"hp11").unwrap()
         && gw.isItemOff(~"hp12").unwrap()
         && gw.isItemOff(~"hp13").unwrap()
         && gw.isItemOff(~"hp21").unwrap()
         && gw.isItemOff(~"hp22").unwrap()
         && gw.isItemOff(~"hp23").unwrap()
         && gw.isItemOn(~"hp31").unwrap()
         && gw.isItemOff(~"hp32").unwrap()
         && gw.isItemOff(~"hp33").unwrap()
    );

    gw.perform_action(~"handb");

    assert!(
            gw.isItemOff(~"hp11").unwrap()
         && gw.isItemOff(~"hp12").unwrap()
         && gw.isItemOff(~"hp13").unwrap()
         && gw.isItemOff(~"hp21").unwrap()
         && gw.isItemOff(~"hp22").unwrap()
         && gw.isItemOff(~"hp23").unwrap()
         && gw.isItemOn(~"hp31").unwrap()
         && gw.isItemOff(~"hp32").unwrap()
         && gw.isItemOff(~"hp33").unwrap()
    );

    gw.perform_action(~"handl");

    assert!(
            gw.isItemOff(~"hp11").unwrap()
         && gw.isItemOff(~"hp12").unwrap()
         && gw.isItemOff(~"hp13").unwrap()
         && gw.isItemOn(~"hp21").unwrap()
         && gw.isItemOff(~"hp22").unwrap()
         && gw.isItemOff(~"hp23").unwrap()
         && gw.isItemOff(~"hp31").unwrap()
         && gw.isItemOff(~"hp32").unwrap()
         && gw.isItemOff(~"hp33").unwrap()
    );
    gw.perform_action(~"handl");

    assert!(
            gw.isItemOn(~"hp11").unwrap()
         && gw.isItemOff(~"hp12").unwrap()
         && gw.isItemOff(~"hp13").unwrap()
         && gw.isItemOff(~"hp21").unwrap()
         && gw.isItemOff(~"hp22").unwrap()
         && gw.isItemOff(~"hp23").unwrap()
         && gw.isItemOff(~"hp31").unwrap()
         && gw.isItemOff(~"hp32").unwrap()
         && gw.isItemOff(~"hp33").unwrap()
    );
    gw.perform_action(~"handl");

    assert!(
            gw.isItemOn(~"hp11").unwrap()
         && gw.isItemOff(~"hp12").unwrap()
         && gw.isItemOff(~"hp13").unwrap()
         && gw.isItemOff(~"hp21").unwrap()
         && gw.isItemOff(~"hp22").unwrap()
         && gw.isItemOff(~"hp23").unwrap()
         && gw.isItemOff(~"hp31").unwrap()
         && gw.isItemOff(~"hp32").unwrap()
         && gw.isItemOff(~"hp33").unwrap()
    );
}


#[test]
fn world_gridworld_perform_action_eye() {
    let mut gw = GridWorld::new();

    // f, f, f, r, r, r, b, b, b, l, l, l

    assert!(
            gw.isItemOn(~"vp11").unwrap()
         && gw.isItemOff(~"vp12").unwrap()
         && gw.isItemOff(~"vp13").unwrap()
         && gw.isItemOff(~"vp21").unwrap()
         && gw.isItemOff(~"vp22").unwrap()
         && gw.isItemOff(~"vp23").unwrap()
         && gw.isItemOff(~"vp31").unwrap()
         && gw.isItemOff(~"vp32").unwrap()
         && gw.isItemOff(~"vp33").unwrap()
    );

    gw.perform_action(~"eyef");

    assert!(
            gw.isItemOff(~"vp11").unwrap()
         && gw.isItemOn(~"vp12").unwrap()
         && gw.isItemOff(~"vp13").unwrap()
         && gw.isItemOff(~"vp21").unwrap()
         && gw.isItemOff(~"vp22").unwrap()
         && gw.isItemOff(~"vp23").unwrap()
         && gw.isItemOff(~"vp31").unwrap()
         && gw.isItemOff(~"vp32").unwrap()
         && gw.isItemOff(~"vp33").unwrap()
    );


    gw.perform_action(~"eyef");

    assert!(
            gw.isItemOff(~"vp11").unwrap()
         && gw.isItemOff(~"vp12").unwrap()
         && gw.isItemOn(~"vp13").unwrap()
         && gw.isItemOff(~"vp21").unwrap()
         && gw.isItemOff(~"vp22").unwrap()
         && gw.isItemOff(~"vp23").unwrap()
         && gw.isItemOff(~"vp31").unwrap()
         && gw.isItemOff(~"vp32").unwrap()
         && gw.isItemOff(~"vp33").unwrap()
    );

    gw.perform_action(~"eyef");

    assert!(
            gw.isItemOff(~"vp11").unwrap()
         && gw.isItemOff(~"vp12").unwrap()
         && gw.isItemOn(~"vp13").unwrap()
         && gw.isItemOff(~"vp21").unwrap()
         && gw.isItemOff(~"vp22").unwrap()
         && gw.isItemOff(~"vp23").unwrap()
         && gw.isItemOff(~"vp31").unwrap()
         && gw.isItemOff(~"vp32").unwrap()
         && gw.isItemOff(~"vp33").unwrap()
    );


    gw.perform_action(~"eyer");

    assert!(
            gw.isItemOff(~"vp11").unwrap()
         && gw.isItemOff(~"vp12").unwrap()
         && gw.isItemOff(~"vp13").unwrap()
         && gw.isItemOff(~"vp21").unwrap()
         && gw.isItemOff(~"vp22").unwrap()
         && gw.isItemOn(~"vp23").unwrap()
         && gw.isItemOff(~"vp31").unwrap()
         && gw.isItemOff(~"vp32").unwrap()
         && gw.isItemOff(~"vp33").unwrap()
    );

    gw.perform_action(~"eyer");

    assert!(
            gw.isItemOff(~"vp11").unwrap()
         && gw.isItemOff(~"vp12").unwrap()
         && gw.isItemOff(~"vp13").unwrap()
         && gw.isItemOff(~"vp21").unwrap()
         && gw.isItemOff(~"vp22").unwrap()
         && gw.isItemOff(~"vp23").unwrap()
         && gw.isItemOff(~"vp31").unwrap()
         && gw.isItemOff(~"vp32").unwrap()
         && gw.isItemOn(~"vp33").unwrap()
    );

    gw.perform_action(~"eyer");

    assert!(
            gw.isItemOff(~"vp11").unwrap()
         && gw.isItemOff(~"vp12").unwrap()
         && gw.isItemOff(~"vp13").unwrap()
         && gw.isItemOff(~"vp21").unwrap()
         && gw.isItemOff(~"vp22").unwrap()
         && gw.isItemOff(~"vp23").unwrap()
         && gw.isItemOff(~"vp31").unwrap()
         && gw.isItemOff(~"vp32").unwrap()
         && gw.isItemOn(~"vp33").unwrap()
    );

    gw.perform_action(~"eyeb");

    assert!(
            gw.isItemOff(~"vp11").unwrap()
         && gw.isItemOff(~"vp12").unwrap()
         && gw.isItemOff(~"vp13").unwrap()
         && gw.isItemOff(~"vp21").unwrap()
         && gw.isItemOff(~"vp22").unwrap()
         && gw.isItemOff(~"vp23").unwrap()
         && gw.isItemOff(~"vp31").unwrap()
         && gw.isItemOn(~"vp32").unwrap()
         && gw.isItemOff(~"vp33").unwrap()
    );

    gw.perform_action(~"eyeb");

    assert!(
            gw.isItemOff(~"vp11").unwrap()
         && gw.isItemOff(~"vp12").unwrap()
         && gw.isItemOff(~"vp13").unwrap()
         && gw.isItemOff(~"vp21").unwrap()
         && gw.isItemOff(~"vp22").unwrap()
         && gw.isItemOff(~"vp23").unwrap()
         && gw.isItemOn(~"vp31").unwrap()
         && gw.isItemOff(~"vp32").unwrap()
         && gw.isItemOff(~"vp33").unwrap()
    );

    gw.perform_action(~"eyeb");

    assert!(
            gw.isItemOff(~"vp11").unwrap()
         && gw.isItemOff(~"vp12").unwrap()
         && gw.isItemOff(~"vp13").unwrap()
         && gw.isItemOff(~"vp21").unwrap()
         && gw.isItemOff(~"vp22").unwrap()
         && gw.isItemOff(~"vp23").unwrap()
         && gw.isItemOn(~"vp31").unwrap()
         && gw.isItemOff(~"vp32").unwrap()
         && gw.isItemOff(~"vp33").unwrap()
    );

    gw.perform_action(~"eyel");

    assert!(
            gw.isItemOff(~"vp11").unwrap()
         && gw.isItemOff(~"vp12").unwrap()
         && gw.isItemOff(~"vp13").unwrap()
         && gw.isItemOn(~"vp21").unwrap()
         && gw.isItemOff(~"vp22").unwrap()
         && gw.isItemOff(~"vp23").unwrap()
         && gw.isItemOff(~"vp31").unwrap()
         && gw.isItemOff(~"vp32").unwrap()
         && gw.isItemOff(~"vp33").unwrap()
    );
    gw.perform_action(~"eyel");

    assert!(
            gw.isItemOn(~"vp11").unwrap()
         && gw.isItemOff(~"vp12").unwrap()
         && gw.isItemOff(~"vp13").unwrap()
         && gw.isItemOff(~"vp21").unwrap()
         && gw.isItemOff(~"vp22").unwrap()
         && gw.isItemOff(~"vp23").unwrap()
         && gw.isItemOff(~"vp31").unwrap()
         && gw.isItemOff(~"vp32").unwrap()
         && gw.isItemOff(~"vp33").unwrap()
    );
    gw.perform_action(~"eyel");

    assert!(
            gw.isItemOn(~"vp11").unwrap()
         && gw.isItemOff(~"vp12").unwrap()
         && gw.isItemOff(~"vp13").unwrap()
         && gw.isItemOff(~"vp21").unwrap()
         && gw.isItemOff(~"vp22").unwrap()
         && gw.isItemOff(~"vp23").unwrap()
         && gw.isItemOff(~"vp31").unwrap()
         && gw.isItemOff(~"vp32").unwrap()
         && gw.isItemOff(~"vp33").unwrap()
    );
}
