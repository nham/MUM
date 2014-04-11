extern crate collections;

pub use collections::HashMap;

use item::{Item,ItemSet};
//use world::{World};

pub mod item;
pub mod world;
mod test;

/*
 * "A schema asserts that if its action is taken when its context
 * conditions are all satisfied, then its result conditions will
 * obtain. The assertion is subject to some auxiliary information
 * that the schema maintains, including a reliability factor and a
 * set of known overriding conditions"
 *
 *  - "A schema makes no assertions about what happens if its action is
 *    taken when its context conditions are not all satisfied"
 *  - "A schema is not a rule that says to take an action when its context
 *    is satisfied; the schema just says what would happen if that were 
 *    done."
 *  - "Satisfying a schema's context is not a prerequisite for being able
 *    to take the designated action..."
 *  - "The schema does not assert that the effects noted in the schema's
 *    result are exhaustive; other events may occur as well, whether or 
 *    not they are caused by the action."
 *
 * So if I may try to summarize the above, a schema says "if <context>
 * conditions all hold when <action> is taken, then <result> conditions
 * will be true (with probability of <reliability>?)"
 *
 *
 * "A schema's correlation is the ratio of the probability with which a
 * transition to the schema's result state obtains when the schema is
 * activated to the frequency with which that transition obtains when the
 * schema is applicable, but not activated"
 *
 * TODO: Add extended context and extended results.
 *
 * "A schema's auxiliary data (including the content of the extended-
 * context and extended-result slots are subject to revision, but a schema's
 * context, action and result uniquely identify that schema and do not 
 * change."
 *
 * p79: "Extended contexts, like extended results, identify relevant items for
 * inclusion in spinoff schemas. Extended contexts serve a second function:
 * identifying overriding conditions, that is, conditions under which an
 * ordinarily reliable schema is invalid." (TODO: is that use of "invalid" inline
 * with the definition of "valid" for schemas?)
 */
struct Schema {
    context: ItemSet,
    action: Action,
    result: ItemSet,
    reliability: f64,
    correlation: f64,
    //overriding: ???,
    //ext_context: HashMap<uint, Slot>,
    ext_result: HashMap<uint, ResultSlot>,
}

/*
 * p72: "These statistics are tabulated over a number of trials in which the action
 * is taken, and a number of trials in which it is not; the more trials there have
 * been, and the more discrepancy there is between the two probabilities [ratio?]
 * the sooner the machinery will detect the difference."
 */
struct ResultSlot {
    // P(slot's item turns On when the schema's action has just been taken) 
    // ---------------------------------------------------------------- 
    // P(slot's item turns On when schema's action has not been taken)
    pt_corr: f64, 

    // P(slot's item turns Off when the schema's action has just been taken) 
    // ---------------------------------------------------------------- 
    // P(slot's item turns Off when schema's action has not been taken)
    nt_corr: f64, 
}

enum Action {
    Primitive(~str),
    Composite(CompositeAction),
}


// TODO: figure out what composite actions are. "A composite action is defined
// with respect to some *goal state*; it is the action of bringing about that
// state... Each composite action has an associated *controller*. Just as a
// schema's extended context and extended result have a slot for every extant
// item, a composite action's controller has a slot for every schema. Each
// slot contains data about whether the schema lies along some chain to the
// goal state, and, if so, the *proximity* to the goal that will be achieved
// if the schema is activated."
struct CompositeAction {
    goal_state: ItemSet,
    controller: Controller
}

struct CompositeActionSlot {
    lies_along_chain: bool,
    proximity: Option<f64>,
}

// one slot for each schema
struct Controller {
    slots: Vec<(uint, CompositeActionSlot)>,
}



/*
 * "A schema's context is said to be *satisfied* when all the positively
 * included items are On and all the negatively included items Off"
 *
 * "A schema is said to be *applicable* when its context is satisfied and
 * no known overriding conditions obtain."
 *
 * "An applicable schema is said to be *valid* at times when its assertion
 * is in fact true--that is, at times when the result would indeed obtain
 * if the action were taken."
 */

// "A synthetic item reifies the validity conditions of its host schema"
impl Schema {
    fn is_applicable(&self) -> bool {
        for i in self.context.iter() {
            if !i.is_satisfied() {
                return false;
            }
        }

        true
    }

    /*
     * To *activate* a schema is to initiate its action when the schema
     * is applicable. A schema asserts that its activation culminates in
     * turning On those items that are positively included in the result,
     * and turning Off those items that are negatively included."
     */
    fn activate(&mut self) {

    }
}

/*

// each item has an associated ID in the "Mechanism". I think this is needed because
// "each schema has two large ancillary structures, an extended context and an
// "extended result. Each has a slot for every item in the schema mechanism.",
// which means we need to have some registry of all the schemas.
struct Mechanism {
    world: World,
    items: Vec<(uint, Item)>,
    actions: Vec<(uint, Action)>,
    schemas: Vec<(uint, Schema)>,
}

impl Mechanism {
    fn new(world: World) -> Mechanism {
        // TODO: I think we are pushing wrong object types here, in all 3 cases
        let mut items: Vec<(uint, Item)> = vec!();

        for pi in world.prim_items().iter() {
            items.push(pi);
        }

        let mut actions: Vec<(uint, Action)> = vec!();

        for pa in world.prim_actions().iter() {
            actions.push(pa);
        }

        // initialize bare schemas, one for each action
        let mut schemas: Vec<(uint, Schema)> = vec!();

        for pa in world.prim_actions().iter() {
            schemas.push(pa);
        }

        Mechanism(world: world, items: items, actions: actions, schemas: schemas)
    }

    fn synthesize_item() {
        // create a new item, somehow!
    }

    fn activate(&mut self, schema_id: uint) {
        let action_id = the id of the action associated with the schema;
        let expl_act: Vec<Schema> = all applicable schemas that have the same action as action_id;

        // I think activation just means, fundamentally, taking some primitive action (or sequence
        // of primitive actions?) fundamentally, actions affect the world.

    }

    // at each time step, select a schema for activation
    fn step(&mut self) {
        let schema_id = select most important schema to activate;
        self.activate(schema_id);

    }
}
*/

/*
 * "Schemas compete for activation. At top level, the schema mechanism selects
 * a schema for activation. Selection occurs at each next time unit in the
 * current, discrete-time implementation... In the present implementation,
 * only one schema is activated at a time. However, the activation of a schema
 * that has a composite action entails the immediate activation of some
 * component schema; thes the current implementation supports nested activat-
 * ions."
 *
 * "The top-level selection process chooses among applicable schemas according
 * to the activation importance they assert. The importance of activating a
 * given schema is based on two criteria: explicit goal-pursuit, and explorat- 
 * ion.
 */
fn main() {

}


