use item::{Item};

use super::{HashMap};

pub trait World {
    fn prim_actions(&self) -> Vec<~str>;
    fn prim_items(&self) -> Vec<Item>;
}

pub struct GridWorld {
    actions: Vec<~str>,
    items: Vec<Item>,
    hand_pos: (uint, uint),
    glance_pos: (uint, uint),
    //item_list: HashMap<~str, uint>, // gives the id of each
}

enum GridDir {
    Forward,
    Backward,
    Right,
    Left,
}

impl GridWorld {
    pub fn new() -> GridWorld {
        let actions = vec!(~"handf", ~"handb", ~"handr", ~"handl",
                           ~"eyef", ~"eyeb", ~"eyer", ~"eyel",
                           ~"grasp", ~"ungrasp");

        let items = vec!();

        let mut id = 0u;

        // hp11-hp33
        let mut hp = vec!();
        for i in range(0, 9) {
            hp.push( Item::new(id) );
            id += 1;
        }

        // vp11-vp33
        let mut vp = vec!();
        for i in range(0, 9) {
            vp.push( Item::new(id) );
            id += 1;
        }

        // tactf-tactl
        let mut tact = vec!();
        for i in range(0, 4) {
            tact.push( Item::new(id) );
            id += 1;
        }

        // text0-text3
        let mut text = vec!();
        for i in range(0, 4) {
            text.push( Item::new(id) );
            id += 1;
        }

        // bodyf-bodyl
        let mut body = vec!();
        for i in range(0, 4) {
            body.push( Item::new(id) );
            id += 1;
        }

        // taste0-taste3
        let mut taste = vec!();
        for i in range(0, 4) {
            taste.push( Item::new(id) );
            id += 1;
        }
        
        // hcl, hgr
        let mut hand = vec!();
        for i in range(0, 2) {
            hand.push( Item::new(id) );
            id += 1;
        }

        // vf00-vf44
        let mut vf = vec!();
        for i in range(0, 25) {
            vf.push( Item::new(id) );
            id += 1;
        }

        // fovf00-fovf33, ..., fovx00-fovx33
        // 16 each for f,b,r,l,x
        let mut fovea = vec!();
        for i in range(0, 80) {
            fovea.push( Item::new(id) );
            id += 1;
        }

        GridWorld { actions: actions, items: items, 
                    hand_pos: (0u, 0u), glance_pos: (0u, 0u) }

    }

    fn perform_action(&mut self, action: ~str) {
        if action.slice(0, 4) == "hand" {
            match action.slice(4, 5) {
                "f" => self.move_hand(Forward),
                "b" => self.move_hand(Backward),
                "r" => self.move_hand(Right),
                "l" => self.move_hand(Left),
                _ => fail!("aghblaghaga"),
            }
        }

    }

    fn move_hand(&mut self, dir: GridDir) {
        let x = self.hand_pos.val0();
        let y = self.hand_pos.val1();
        match dir {
            Forward => { self.hand_pos = (GridWorld::inc_grid_pos(x), y); },
            Backward => { self.hand_pos = (GridWorld::dec_grid_pos(x), y); },
            Right => { self.hand_pos = (x, GridWorld::inc_grid_pos(y)); },
            Left => { self.hand_pos = (x, GridWorld::dec_grid_pos(y)); },
        }
    }

    fn move_glance(&mut self, dir: GridDir) {
        let x = self.glance_pos.val0();
        let y = self.glance_pos.val1();
        match dir {
            Forward => { self.glance_pos = (GridWorld::inc_grid_pos(x), y); },
            Backward => { self.glance_pos = (GridWorld::dec_grid_pos(x), y); },
            Right => { self.glance_pos = (x, GridWorld::inc_grid_pos(y)); },
            Left => { self.glance_pos = (x, GridWorld::dec_grid_pos(y)); },
        }
    }

    fn inc_grid_pos(pos: uint) -> uint {
        if pos == 3 {
            3
        } else {
            pos + 1
        }
    }

    fn dec_grid_pos(pos: uint) -> uint {
        if pos == 1 {
            1
        } else {
            pos - 1
        }
    }
}

impl World for GridWorld {
    fn prim_actions(&self) -> Vec<~str> {
        self.actions
    }

    fn prim_items(&self) -> Vec<Item> {
        self.items
    }
}
