pub trait World {
    fn prim_actions() -> Vec<~str>;
    fn prim_items() -> Vec<~str>;
}

struct GridWorld

impl World for GridWorld {
    // p116
    fn prim_actions() -> Vec<~str> {
        vec!(~"handf", ~"handb", ~"handr", ~"handl",
             ~"eyef", ~"eyeb", ~"eyer", ~"eyel",
             ~"grasp", ~"ungrasp");

    }

    // p117
    fn prim_items() -> Vec<~str> {
        // hp: haptic-proprioceptive. represents which position the hand is in
        //     hpxy means x-th column from the left, y-th row from the bottom
        // vp: visual-proprioceptive. represents glance orientation. same
        //     position arrangement as for hp
        vec!(~"hp11", ~"hp12", ~"hp13", ~"hp21", ~"hp22", ~"hp23", ~"hp31", ~"hp32", ~"hp33",
            ~"vp11", ~"vp12", ~"vp13", ~"vp21", ~"vp22", ~"vp23", ~"vp31", ~"vp32", ~"vp33",
            ~"tactf", ~"tactb", ~"tactr", ~"tactl",
            ~"text0", ~"text1", ~"text2", ~"text3",
            ~"bodyf", ~"bodyb", ~"bodyr", ~"bodyl",
            ~"taste0", ~"taste1", ~"taste2", ~"taste3",
            ~"hcl", ~"hgr",
        //TODO everything remaining on p117

    }
}
