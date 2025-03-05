#![allow(unused, dead_code)]

use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize)]
pub struct Bisection<I: Clone, S> {
    /// I is the index type, something short and clonable
    objects: Vec<I>,
    /// state that handles the various object types
    /// if your object type isnt clone you might keep it here and use
    /// numeric indices
    state: S,
}
