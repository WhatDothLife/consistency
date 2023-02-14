use crate::problem::*;

/// The SAC-1 algorithm due to Bessiere and Debruyne 1997.
pub fn sac_1<P, AC>(problem: &mut P, ac: AC) -> bool
where
    P: Problem + Clone,
    AC: Fn(&mut P) -> bool,
{
    if ac(problem) {
        return false;
    }

    let mut changed = true;

    while changed {
        changed = false;

        for x in problem.variables() {
            for a in problem.values(x) {
                let mut problem_x_a = problem.clone();
                problem_x_a.set(x, a);

                if !ac(&mut problem_x_a) {
                    problem.remove(x, a);
                    changed = true;
                };
            }
            if problem.values(x).is_empty() {
                return false;
            }
        }
    }
    true
}

// /// The SAC-Opt algorithm due to Bessiere and Debruyne 2008.
// ///
// /// Returns false, if an empty domain is derived for some vertex v, true otherwise.
// pub fn sac_opt<P, AC>(problem: &mut P, ac: AC) -> bool
// where
//     P: Problem + Clone,
//     AC: Fn(&mut P) -> bool,
// {
//     if ac(problem) {
//         return false;
//     }

//     let mut pending_list = Vec::new();
//     let mut ds = Vec::<Vec<P>>::new();          //(X, A) -> P
//     let mut q = Vec::<Vec<Vec<(X, A)>>>::new(); //(X, A) -> [(X, A)]

//     // Init phase
//     for i in problem.variables() {
//         for a in problem.values(i) {
//             let mut problem_i_a = problem.clone();
//             problem_i_a.set(i, a);
//             ds[i][a] = problem_i_a;

//             let mut set = Vec::<(X, A)>::new();
//             for b in problem.values(i) {
//                 if b != a {
//                     set.push((i, b));
//                 }
//             }
//             q[i][a] = set;
//             pending_list.push((i, a));
//         }
//     }

//     // Propag phase
//     while let Some((i, a)) = pending_list.pop() {
//         let p = &mut ds[i][a];
//         for (x, y) in q[i][a] {
//             p.remove(x, y);
//         }
//         if ac(p) {
//             q[i][a].clear();
//         } else {
//             problem.remove(i, a);
//             if problem.values(i).is_empty() {
//                 return false;
//             }
//             for (j, vj) in ds.iter().enumerate() {
//                 for (b, m) in vj.iter().enumerate() {
//                     if m.remove(i, a) {
//                         q[j][b].push((i, a));
//                         pending_list.push((j, b));
//                     }
//                 }
//             }
//         }
//     }
//     true
// }
