#![forbid(missing_docs)]
//! A test to see if `#[automatically_derived]` is sufficient to silent `forbid(missing_docs)`.
use bevy::prelude::*;
use bevy::ecs::query::WorldQuery;

/// A dummy component to query for.
#[derive(Clone, Copy, Default, Debug, Component)]
pub struct DummyComponent;

/// A dummy query on which we will derive [`WorldQuery`].
#[derive(WorldQuery)]
pub struct DummyQuery {
    /// Gotta query 'em all!
    dummy: &'static DummyComponent,
}

fn main() {
    println!("Hello, world!");
}
