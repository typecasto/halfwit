//! Search for objects with dominant behavior.
//!
//! # Problem
//! You have:
//! - A set `U` containing several objects, where each object in the set can
//! exhibit one of two kinds of behavior: "dominant", and "recessive".
//! - A test that can be ran on some subset of `U` (denoted `T`). This test must
//! exhibit some behavior `R` if the test set `T` contains only objects with
//! recessive behavior
//!
//! # Examples
//! - A game with many mods that is doing something unwanted, and
//! it's unknown which of the mods is causing the behavior (lag, crashes, etc).
//!
//! # Solution
//! For some subset of objects, starting with the entire set, run the test on
//! the subset. If the test is recessive, all objects in that set are recessive,
//! stop searching. If the test is dominant, cut the subset into two halves, and
//! search each half recursively. If the subset consists of a single object,
//! instead of cutting it in half, simply mark it dominant and return.
//!
//! Basically, it's depth first search, with early branch pruning, and multiple
//! search targets which are specified by behavior they cause in a group.

use std::{collections::VecDeque, ops::RangeInclusive};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum State {
    Enabled,
    Disabled,
}

/// Can be enabled or disabled.
pub trait Stateful {
    /// Sets the state of an object, updating the "real" thing it represents.
    fn set_state(&self, state: &State);
    /// Gets the current cached state of an object.
    fn state(&self) -> State;
    // /// Verifies the uncached state of the object, if possible.
    // fn verify_state(&self) -> Option<State>;
}

#[derive(PartialEq, Eq, Clone, Copy, Default, Debug)]
/// What effect an object has on the behavior of a test.
///
/// For some given test set, if any object is [Behavior::Dominant], the test
/// will exhibit dominant behavior. If and only if the test set is solely
/// [Behavior::Recessive] objects, the test will then result in recessive
/// behavior.
///
/// The dominant behavior is usually some program crashing or failing, caused by
/// a dominant object that is broken, and recessive behavior is usually the same
/// program functioning normally, where all objects are recessive (functioning).
pub enum Behavior {
    #[default]
    Unknown,
    Recessive,
    Dominant,
}

// pub struct Entry<T: Stateful> {
//     object: T,
//     behavior: Behavior,
// }

/// A pair of indices representing the behavior of some items in a bisection.
///
/// `from` and `to` point to indices in the [Bisection] object, and are inclusive.
/// It is an error to construct a Group such that `to > from`.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Group {
    from: usize,
    to: usize,
    behavior: Behavior,
}

impl Group {
    pub fn new(from: usize, to: usize) -> Self {
        Group {
            from,
            to,
            behavior: Behavior::Unknown,
        }
    }

    pub fn from(&self) -> usize {
        self.from
    }

    pub fn to(&self) -> usize {
        self.to
    }

    pub fn behavior(&self) -> Behavior {
        self.behavior
    }

    pub fn set_behavior(&mut self, behavior: Behavior) {
        self.behavior = behavior;
    }

    /// Splits this group into two groups
    ///
    /// # Examples
    /// ```
    /// # use halfwit::bisection::Group;
    /// assert_eq!(
    ///     Group::new(7, 8).split(),
    ///     (Group::new(7, 7), Group::new(8, 8))
    /// )
    /// ```
    pub fn split(&self) -> (Self, Self) {
        if self.from == self.to {
            panic!("Tried to split a group that only contains one object!")
        }
        // If we're recessive, we really shouldn't be splitting anyways, but it's good
        // to maintain that knowledge.
        let new_behavior = match self.behavior {
            Behavior::Recessive => Behavior::Recessive,
            Behavior::Dominant | Behavior::Unknown => Behavior::Unknown,
        };
        let g1 = Self {
            from: self.from,
            to: self.from + (self.to - self.from) / 2,
            behavior: new_behavior,
        };
        let g2 = Self {
            from: g1.to + 1,
            to: self.to,
            behavior: new_behavior,
        };
        (g1, g2)
    }

    /// Get the number of objects this Group contains
    ///
    /// # Examples
    /// ```
    /// # use halfwit::bisection::Group;
    /// assert_eq!(Group::new(1, 1).size(), 1);
    /// assert_eq!(Group::new(1, 2).size(), 2);
    /// assert_eq!(Group::new(2, 3).size(), 2);
    /// ```
    pub fn size(&self) -> usize {
        self.to - self.from + 1
    }
}

// Allows the `for x in &group {}` syntax
impl IntoIterator for &Group {
    type Item = usize;
    type IntoIter = RangeInclusive<Self::Item>;
    /// Iterate through indices in the group
    /// 
    /// # Examples
    /// ```
    /// # use halfwit::bisection::Group;
    /// let group = Group::new(3, 7);
    /// let mut test_vec = Vec::new();
    /// for x in &group {
    ///     test_vec.push(x * 10);
    /// }
    /// assert_eq!(
    ///     test_vec,
    ///     vec![30, 40, 50, 60, 70]
    /// );
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.from()..=self.to()
    }
}

/// An active bisection taking place
///
/// Holds no reference to the actual behavior to be tested, that's done
/// elsewhere.
pub struct Bisection<T: Stateful> {
    objects: Vec<T>,
    groups: Vec<Group>,
}

impl<T: Stateful> Bisection<T> {
    pub fn new(objects: Vec<T>) -> Self {
        // one group covering all objects to start
        Self {
            groups: vec![Group::new(0, objects.len() - 1)],
            objects,
        }
    }

    /// Change the state of all elements in a [Group]
    pub fn set_group_state(&mut self, group: &Group, state: State) {
        for i in group {
            self.objects.get_mut(i).unwrap().set_state(&state);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // useful macro to have
    macro_rules! test_assert_eq {
        ($name:ident, $a:stmt, $b:stmt) => {
            #[test]
            fn $name() {
                assert_eq!({ $a }, { $b });
            }
        };
    }

    // and we should probably test the macro itself too
    test_assert_eq!(macro_test_add, 2 + 2, 4);
    test_assert_eq!(
        macro_test_block,
        {
            let mut x = 2;
            x += 2;
            x
        },
        4
    );
    // nice!

    /// Splitting a Recessive group should result in 2 recessive groups
    #[test]
    fn group_split_behavior() {
        let mut jef = Group::new(7, 8);
        jef.behavior = Behavior::Recessive;
        assert_eq!(
            jef.split(),
            (
                Group {
                    from: 7,
                    to: 7,
                    behavior: Behavior::Recessive
                },
                Group {
                    from: 8,
                    to: 8,
                    behavior: Behavior::Recessive
                }
            )
        )
    }
}
