pub type X = usize;
pub type A = usize;

pub trait Problem {
    /// Returns the number of variables in the problem.
    fn num_vars(&self) -> usize;

    /// Returns a vector of the variables in the problem.
    fn variables(&self) -> Vec<X>;

    /// Returns a vector of values that can be assigned to a given variable `x`.
    fn values(&self, x: X) -> Vec<A>;

    /// Removes a value `a` from the list of possible values for a given variable `x`.
    fn remove(&mut self, x: X, a: A);

    /// Sets a value `a` for a given variable `x`.
    fn set(&mut self, x: X, a: A);

    /// Returns a vector of arcs, where each arc is a tuple of two variables
    /// `(x, y)` representing a constraint between the two variables.
    fn arcs(&self) -> Vec<(X, X)>;

    /// Returns a boolean indicating whether a given assignment `u` is
    /// consistent with another assignment `v` with respect to the constraints
    /// defined by the arcs.
    fn check(&self, u: (X, A), v: (X, A)) -> bool;
}
