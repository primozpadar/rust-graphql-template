pub mod placeholder;

use juniper::{EmptySubscription, RootNode};

use crate::db::Pool;
pub struct Context {
  pub pool: Pool,
}
impl juniper::Context for Context {}

pub struct Query;
pub struct Mutation;
pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
  Schema::new(Query, Mutation, EmptySubscription::new())
}
