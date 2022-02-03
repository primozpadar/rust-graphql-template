use juniper::{EmptySubscription, FieldResult, RootNode};

use crate::db::Pool;
use crate::handlers::placeholder::{NewPlaceholder, Placeholder};

pub struct Context {
  pub pool: Pool,
}
impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
  #[graphql(description = "Placeholder query")]
  fn placeholders(context: &Context) -> FieldResult<Vec<Placeholder>> {
    let conn = context.pool.get().unwrap();
    Ok(Placeholder::get_all(&conn)?)
  }
}

pub struct MutationRoot;

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
  #[graphql(description = "Placeholder mutation")]
  fn create_placeholder(context: &Context, new_placeholder: NewPlaceholder) -> FieldResult<Placeholder> {
    let conn = context.pool.get().unwrap();
    Ok(Placeholder::create(&conn, new_placeholder)?)
  }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
  Schema::new(QueryRoot, MutationRoot, EmptySubscription::new())
}
