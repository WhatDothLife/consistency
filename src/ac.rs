use std::iter::FromIterator;

use crate::problem::*;

fn revise<P: Problem>(x: X, y: X, problem: &mut P) -> Option<bool> {
    let mut mutated = false;

    for ax in problem.values(x) {
        let mut is_possible = false;

        for ay in problem.values(y) {
            if problem.check((x, ax), (y, ay)) {
                is_possible = true;
                break;
            }
        }

        if !is_possible {
            problem.remove(x, ax);
            mutated = true;

            if problem.values(x).is_empty() {
                return None;
            }
        }
    }

    Some(mutated)
}

/// The AC-1 algorithm due to Mackworth 1977.
pub fn ac_1<P: Problem>(problem: &mut P) -> bool {
    let mut changed = true;

    while changed {
        changed = false;

        for (x, y) in problem.arcs() {
            if let Some(res) = revise(x, y, problem) {
                if res {
                    changed = true;
                }
            } else {
                return false;
            }
        }
    }
    true
}

/// The AC-3 algorithm due to Mackworth 1977.
pub fn ac_3<P: Problem>(problem: &mut P) -> bool {
    let mut work_list = Vec::from_iter(problem.arcs());
    let mut neighbors = vec![Vec::new(); problem.num_vars()];

    for arc in problem.arcs() {
        neighbors[arc.1].push(arc);
    }

    while let Some((x, y)) = work_list.pop() {
        if let Some(res) = revise(x, y, problem) {
            if res {
                work_list.extend(neighbors[x].iter().copied());
            }
        } else {
            return false;
        }
    }
    true
}
