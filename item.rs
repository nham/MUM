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
    fn isOn(&self) {
        self == On
    }

    fn isOff(&self) {
        self == Off
    }
}

// "An *item* is a state element. Each item represents some condition in the
// world, and has a state of On or Off to assert respectively that the
// condition does or does not currently obtain..."
pub struct Item {
    id: uint,
    state: ItemState,
    generality: f64, // "rate of being On rather than Off"
    accessibility: f64, // "rate of being at the end of some chain of schemas"
    primitiveValue: f64 // "Built-in positive or negative desirability measure"
    delegatedValue: f64 // "Acquired positive or negative desirability measure"
}

impl Item {
    fn is_satisfied(&self) -> bool {
        (self.negated && self.state.isOff()) 
        || (!self.negated && self.state.isOn())
    }
}

struct IncludedItem {
    item: Item,
    negated: bool,
}


// Structure for contexts and results.
pub type ItemSet = Vec<IncludedItem>;

