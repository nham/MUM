/*
 * "There are two kinds of items, primitive and synthetic. Primitive items
 * are build into the schema mechanism--they are part of its initial endowment."
 */

#[deriving(Eq)]
enum ItemState {
    On,
    Off,
    Unknown,
}

impl ItemState {
    fn isOn(self) -> bool {
        self == On
    }

    fn isOff(self) -> bool {
        self == Off
    }
}

// "An *item* is a state element. Each item represents some condition in the
// world, and has a state of On or Off to assert respectively that the
// condition does or does not currently obtain..."
pub struct Item {
    state: ItemState,
    generality: f64, // "rate of being On rather than Off"
    accessibility: f64, // "rate of being at the end of some chain of schemas"
    primitiveValue: f64, // "Built-in positive or negative desirability measure"
    delegatedValue: f64, // "Acquired positive or negative desirability measure"
}

impl Item {
    pub fn new() -> Item {
        Item { state: Off, 
               generality: 0f64, accessibility: 0f64, 
               primitiveValue: 0f64, delegatedValue: 0f64 }
    }

}

struct IncludedItem {
    item: Item,
    negated: bool,
}

impl IncludedItem {
    pub fn is_satisfied(&self) -> bool {
        (self.negated && self.item.state.isOff()) 
        || (!self.negated && self.item.state.isOn())
    }
}


// Structure for contexts and results.
pub type ItemSet = Vec<IncludedItem>;

