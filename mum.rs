use item::{Item,ItemSet};

pub mod item

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
 */
struct Schema {
    context: Vec<Item>,
    action: Event,
    result: Vec<Item>,
    reliability: f64,
    correlation: f64,
    overriding: ???,
    ext_context: HashMap<uint, Slot>,
    ext_result: HashMap<uint, Slot>,
}

type Event = ~str;


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


// TODO: figure out what composite actions are. "A composite action is defined
// with respect to some *goal state*; it is the action of bringing about that
// state... Each composite action has an associated *controller*. Just as a
// schema's extended context and extended result have a slot for every extant
// item, a composite action's controller has a slot for every schema. Each
// slot contains data about whether the schema lies along some chain to the
// goal state, and, if so, the *proximity* to the goal that will be achieved
// if the schema is activated."
struct CompositeAction {
    controller: Controller
}

struct Controller {
    slots: Vec<Slot>,
}


// each item has an associated ID in the "Mechanism". I think this is needed because
// "each schema has two large ancillary structures, an extended context and an
// "extended result. Each has a slot for every item in the schema mechanism.",
// which means we need to have some registry of all the schemas.
struct Mechanism {
    items: Vec<(uint, Item)>,
    actions: Vec<(uint, Action)>,
    schemas: Vec<(uint, Schema)>,
}

impl Mechanism {
    fn new() -> Mechanism {
        // initialize all primitive items
    }

    fn synthesize_item() {
        // create a new item, somehow!
    }

    fn activate(&mut self, schema_id: uint) {
        let action_id = the id of the action associated with the schema;
        let expl_act: Vec<Schema> = all applicable schemas that have the same action as action_id;

    }

    // at each time step, select a schema for activation
    fn step(&mut self) {
        let schema_id = select most important schema to activate;
        self.activate(schema_id);

    }
}

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


