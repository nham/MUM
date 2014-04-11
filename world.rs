use item::{Item, On, Off};

use super::{HashMap};

/*
pub trait World {
    fn prim_actions(&self) -> Vec<~str>;
    fn prim_items(&self) -> Vec<Item>;
}
*/

pub struct GridWorld {
    actions: Vec<~str>,
    pub items: HashMap<~str, Item>,
    hand_pos: (uint, uint),
    glance_pos: (uint, uint),
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

        let mut items = HashMap::new();


        // hp11-hp33
        for i in range(1, 4) {
            for j in range(1, 4) {
                items.insert(~"hp" + i.to_str() + j.to_str(), Item::new());
            }
        }

        // vp11-vp33
        for i in range(1, 4) {
            for j in range(1, 4) {
                items.insert(~"vp" + i.to_str() + j.to_str(), Item::new());
            }
        }

        // tactf-tactl
        items.insert(~"tactf", Item::new());
        items.insert(~"tactb", Item::new());
        items.insert(~"tactr", Item::new());
        items.insert(~"tactl", Item::new());

        // text0-text3
        for i in range(0, 4) {
            items.insert(~"text" + i.to_str(), Item::new());
        }

        // bodyf-bodyl
        items.insert(~"bodyf", Item::new());
        items.insert(~"bodyb", Item::new());
        items.insert(~"bodyr", Item::new());
        items.insert(~"bodyl", Item::new());

        // taste0-taste3
        for i in range(0, 4) {
            items.insert(~"taste" + i.to_str(), Item::new());
        }
        
        // hcl, hgr
        items.insert(~"hcl", Item::new());
        items.insert(~"hgr", Item::new());

        // vf00-vf44
        for i in range(0, 5) {
            for j in range(0, 5) {
                items.insert(~"vf" + i.to_str() + j.to_str(), Item::new());
            }
        }

        // fovf00-fovf33, ..., fovx00-fovx33
        // 16 each for f,b,r,l,x
        for i in range(0, 4) {
            for j in range(0, 4) {
                items.insert(~"fovf" + i.to_str() + j.to_str(), Item::new());
            }
        }
        for i in range(0, 4) {
            for j in range(0, 4) {
                items.insert(~"fovb" + i.to_str() + j.to_str(), Item::new());
            }
        }
        for i in range(0, 4) {
            for j in range(0, 4) {
                items.insert(~"fovr" + i.to_str() + j.to_str(), Item::new());
            }
        }
        for i in range(0, 4) {
            for j in range(0, 4) {
                items.insert(~"fovl" + i.to_str() + j.to_str(), Item::new());
            }
        }
        for i in range(0, 4) {
            for j in range(0, 4) {
                items.insert(~"fovx" + i.to_str() + j.to_str(), Item::new());
            }
        }

        // initialize some items
        items.find_mut(&~"hp11").unwrap().setOn();

        GridWorld { actions: actions, items: items, 
                    hand_pos: (0u, 0u), glance_pos: (0u, 0u) }
    }


    pub fn perform_action(&mut self, action: ~str) {
        if action.slice(0, 4) == "hand" {
            let key = ~"hp" + GridWorld::coords_tuple_to_str(self.hand_pos);
            self.turnItemOff(key);

            match action.slice(4, 5) {
                "f" => self.move_hand(Forward),
                "b" => self.move_hand(Backward),
                "r" => self.move_hand(Right),
                "l" => self.move_hand(Left),
                _ => fail!("aghblaghaga"),
            }

            let key = ~"hp" + GridWorld::coords_tuple_to_str(self.hand_pos);
            self.turnItemOn(key);
        }

    }

    // return None if the item doesn't exist
    pub fn isItemOn(&self, name: ~str) -> Option<bool> {
        let find = self.items.find(&name);

        match self.items.find(&name) {
            None => None,
            Some(item) => Some(item.isOn()),
        }
    }


    // return None if the item doesn't exist
    pub fn isItemOff(&self, name: ~str) -> Option<bool> {
        let find = self.items.find(&name);

        match self.items.find(&name) {
            None => None,
            Some(item) => Some(item.isOff()),
        }
    }


    fn turnItemOff(&mut self, name: ~str) {
        let item_ref = self.items.find_mut(&name).unwrap();
        item_ref.setOff();
    }

    fn turnItemOn(&mut self, name: ~str) {
        let item_ref = self.items.find_mut(&name).unwrap();
        item_ref.setOn();
    }

    fn coords_tuple_to_str(tup: (uint, uint)) -> ~str {
        tup.val0().to_str() + tup.val1().to_str()
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

/*
impl World for GridWorld {
    fn prim_actions(&self) -> Vec<~str> {
        self.actions
    }

    fn prim_items(&self) -> Vec<Item> {
        self.items
    }
}
*/
