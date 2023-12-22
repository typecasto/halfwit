use std::default;
use std::ops::ControlFlow;

pub struct Bisection<A: Actionable<B>, B> {
    base: B,
    current: B,
    actions: Vec<A>
}

/// A set of potential actions to apply to some object.
/// 
/// If the information is kept track of elsewhere, such as in the filesystem,
/// B can very well just be `()`
pub struct ActionSet<A: Actionable<B>, B> {
    // Pool of actions to search through.
    action_pool: Vec<A>,
    current: B
}

/// An individual action
pub trait Actionable<B> {
    /// Mutates B in some way
    fn apply(&self, base: B) -> B;
    /// Unapplies this action, or maybe all actions.
    /// 
    /// Returns a modified version of `current`.
    /// 
    /// Some usecases require individual actions to be unapplied one-at-a-time.
    /// If this matches your usecase, for instance adding or removing items from
    /// a collection, you should implement this function such that it unapplies
    /// `self` from `base`, and returns [ControlFlow::Continue].
    /// 
    /// Other usecases are better suited by unapplying all actions at once, like
    /// applying patches to a text file. In this case, you should implement this
    /// function so that it ignores `self` and instead unapplies all actions at
    /// once from `current`, and returns [ControlFlow::Break].
    /// 
    /// For some usecases, it may be sufficient to simply return `B.default()`;
    fn unapply(&self, current: B) -> ControlFlow<B, B>;
}

impl<T: Clone> Actionable<Vec<T>> for T {
    fn apply(&self, mut base: Vec<T>) -> Vec<T> {
        base.push(self.clone());
        base
    }
    fn unapply(&self, current: Vec<T>) -> ControlFlow<Vec<T>, Vec<T>> {
        ControlFlow::Break(Vec::default())
    }
}
