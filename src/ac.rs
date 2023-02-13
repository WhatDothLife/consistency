use crate::problem::*;

pub enum Revision {
    Unchanged,
    Changed,
    Empty,
}

fn revise<P: Problem>(x: X, y: X, problem: &mut P) -> Revision {
    let mut revision = Revision::Unchanged;

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
            revision = Revision::Changed;

            if problem.values(x).is_empty() {
                return Revision::Empty;
            }
        }
    }

    revision
}

/// The AC-1 algorithm due to Mackworth 1977.
pub fn ac_1<P: Problem>(problem: &mut P) -> bool {
    let mut changed = true;

    while changed {
        changed = false;

        for (x, y) in problem.arcs() {
            match revise(x, y, problem) {
                Revision::Unchanged => {}
                Revision::Changed => changed = true,
                Revision::Empty => return false,
            }
        }
    }
    true
}

/// The AC-3 algorithm due to Mackworth 1977.
pub fn ac_3<P: Problem>(problem: &mut P) -> bool {
    let mut work_list = problem.arcs();
    let mut neighbors = vec![vec![]; problem.num_vars()];

    for (x, y) in problem.arcs() {
        neighbors[y].push((x, y));
    }

    while let Some((x, y)) = work_list.pop() {
        match revise(x, y, problem) {
            Revision::Unchanged => {}
            Revision::Changed => work_list.extend_from_slice(&neighbors[x]),
            Revision::Empty => return false,
        }
    }
    true
}
