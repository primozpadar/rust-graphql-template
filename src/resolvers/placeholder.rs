use juniper::FieldResult;

use crate::entities::placeholder::{NewPlaceholder, Placeholder};
use crate::resolvers::{Context, Mutation, Query};

#[juniper::graphql_object(Context = Context)]
impl Query {
  #[graphql(description = "Placeholder query")]
  fn placeholders(context: &Context) -> FieldResult<Vec<Placeholder>> {
    let conn = context.pool.get().unwrap();
    Ok(Placeholder::get_all(&conn)?)
  }
}

#[juniper::graphql_object(Context = Context)]
impl Mutation {
  #[graphql(description = "Placeholder mutation")]
  fn create_placeholder(context: &Context, new_placeholder: NewPlaceholder) -> FieldResult<Placeholder> {
    let conn = context.pool.get().unwrap();
    Ok(Placeholder::create(&conn, new_placeholder)?)
  }
}
