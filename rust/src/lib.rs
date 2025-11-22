use std::ops::{Deref, DerefMut};

use surrealdb::{Connection, Surreal};

mod query;

pub struct Nebula<C: Connection>(Surreal<C>);
impl<C: Connection> Deref for Nebula<C> {
    type Target = Surreal<C>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<C: Connection> DerefMut for Nebula<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}