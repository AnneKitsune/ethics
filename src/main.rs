use std::collections::HashMap;
use std::iter::FromIterator;
// Examples:
// world --(1)> Censorship --(0.6)> people feel controlled --(-0.2)> wellbeing
// Exploitation of resources(5) -> lack of resources -> competition -> wellbeing decrease
//
// Something gets applied only if World points to it.
// World -> Censorship (2)
//
// All values are between -5 and 5
// Chains are multiplicative.
// End effects are additive.

// value pool -> transitions -> value pool -> transitions -> .. -> value pool.
// basically, facts (values) and rules (transitions), applied recursively.

// entities have attributes
// transitions have both a chance of being true (the guessed chance of an event happening) and
// a strength (how much this affects the affected value)
//
//
// world(1) -> inequality(1) -> wellbeing(-4)
//
//
//values are not capped.
//
//
//
//extra feature: generate a .dot graph of the tree.
//
//have:
// - one global value accumulator
// - one temporary state that leads into the next state
//
// also add quantitative measures on which wellbeing is being affected and in which quantity.
// for example:
// wellbeing_animals
// wellbeing_humans
// where both affect global wellbeing in the end
//
//
// ability to construct the global Transitions set using smaller sets.
// For example, one set for capitalism, one for communism, etc etc.

#[derive(Clone, Debug)]
pub struct Values {
    pub values: HashMap<&'static str, f32>,
}

impl Default for Values {
    fn default() -> Self {
        Self {
            values: HashMap::from_iter(vec![
                ("world", 0.0),
                ("wellbeing", 0.0),
                ("wellbeing_humans", 0.0),
                ("wellbeing_animals", 0.0),
                ("inequality", 0.0),
                ("censorship", 0.0),
                ("controlled_feeling", 0.0),
                ("resource_exploitation", 0.0),
                ("resource_usable", 0.0),
                ("resource_depletion", 0.0),
                ("competition", 0.0),
                ("complexity", 0.0),
                ("health", 0.0),
                ("health_cost", 0.0),
                ("environment_quality", 0.0),
            ]),
        }
    }
}

pub struct Transitions {
    // Value into other value with multiplier.
    pub transitions: Vec<(&'static str, &'static str, f32)>,
}

impl std::ops::Add<Transitions> for Transitions {
    type Output = Self;
    fn add(self, other: Transitions) -> Self {
        let mut tr = self.transitions;
        for e in other.transitions {
            tr.push(e);
        }
        Self {
            transitions: tr,
        }
    }
}

fn main() {
    let mut global = Values::default();
    *global.values.get_mut("world").unwrap() = 1.0;
    let mut frame = global.clone();

    let combiners_tr = Transitions {
        transitions: vec! [
            // combine wellbeings
            ("wellbeing_humans", "wellbeing", 1.0),
            ("wellbeing_animals", "wellbeing", 1.0),
        ],
    };
    let censorship_tr = Transitions {
        transitions: vec! [
            // censorship
            ("world", "censorship", 1.0),
            ("censorship", "controlled_feeling", 0.6),
            ("controlled_feeling", "wellbeing_humans", -0.2),
        ],
    };
    let resource_exp_tr = Transitions {
        transitions: vec! [
            // Resource exploitation
            ("world", "resource_exploitation", 0.5),
            ("resource_exploitation", "environment_quality", -0.8),
            ("environment_quality", "health", -0.3), // shit env -> shit health
            ("environment_quality", "wellbeing_humans", -0.2), // shit env -> lowers morale
            ("resource_exploitation", "resource_usable", 1.0),
            ("resource_exploitation", "resource_depletion", 0.2),
            ("resource_depletion", "resource_usable", -0.2),
        ],
    };
    let health_tr = Transitions {
        transitions: vec! [
            // health
            ("resource_usable", "health_cost", 0.5),
            ("health_cost", "health", -1.0),
            ("health", "wellbeing_humans", 1.0),
            ("health", "wellbeing_animals", 0.5),
        ],
    };
    /*let combiners = Transitions {
        transitions: vec! [
        ],
    };*/
    let transitions = combiners_tr + censorship_tr + resource_exp_tr + health_tr;

    for _ in 0..10 {
        frame = apply_transition(&frame, &transitions, &mut global);
    }

    //println!("{:?}", original_values);
    println!("{:?}", global);
}

pub fn apply_transition(frame: &Values, transitions: &Transitions, global: &mut Values) -> Values {
    let mut ret = Values::default();
    for t in &transitions.transitions {
        if ret.values.contains_key(t.0) && ret.values.contains_key(t.1) {
            **ret.values.get_mut(t.1).as_mut().unwrap() += (frame.values.get(t.0).unwrap()) * t.2 as f32;//.min(-5.0).max(5.0);
            **global.values.get_mut(t.1).as_mut().unwrap() += (frame.values.get(t.0).unwrap()) * t.2 as f32;
        }
    }
    ret
}
