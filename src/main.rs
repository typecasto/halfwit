use std::collections::VecDeque;

fn main() {
    let objects = [1, 5, 8, 7, 4, 6, 3, 9, 0, 5, 0, 3, 1, 7, 8, 7, 4, 0, 3];
    // [[0, 1, 2...]] but as a queue of vecs
    // this is the list of sets to test
    let mut queue = VecDeque::from(vec![(0..objects.len()).collect::<Vec<_>>()]);
    println!("{:?}", objects);
    while let Some(set_indices) = queue.pop_front() {
        // set enabled items
        // this will be a call to BiSet::set_enabled(list: &[usize])
        // todo: use references here? that would assert immutability though...
        let mut set_elements = vec![];
        for &idx in set_indices.iter() {
            set_elements.push(objects[idx]);
        }
        // run the test
        // this will be a call to BiSet::perform_test() -> bool
        let behavior_found = set_elements.iter().product::<i32>() == 0;
        // determine next things to print
        if behavior_found {
            assert_ne!(set_indices.len(), 0);
            if set_elements.len() == 1 {
                println!(
                    "Bad element found at index {}: {}",
                    set_indices[0], set_elements[0]
                );
            } else {
                let (a, b) = set_indices.split_at(set_indices.len() / 2);
                queue.push_back(a.to_vec());
                queue.push_back(b.to_vec());
            }
        }
    }
}

/// Contains a set of bisectable objects, and their associated behavior.
trait Bisectable {
    /// Set the list of indices which should be enabled.
    fn set_enabled(&mut self, enabled: &[i32]);
    /// perform a test, returns true if special behavior is found
    fn perform_test(&mut self) -> bool;
}

/// Structure for testing.
///
/// Vector of [i32]s, where the special elements are zeros.
/// The behavior being tested is taking the product and comparing to zero,
/// the result will be 0 iff one of the elements being multiplied is zero.
struct DebugBisectable {
    data: Vec<i32>,
    enabled: Vec<bool>,
}

impl From<Vec<i32>> for DebugBisectable {
    fn from(value: Vec<i32>) -> Self {
        DebugBisectable {
            enabled: vec![true; value.len()],
            data: value,
        }
    }
}
