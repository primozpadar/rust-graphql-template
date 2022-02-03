use diesel::prelude::*;
use juniper::{GraphQLInputObject, GraphQLObject};

use crate::db::{DBResult, PoolConn};
use crate::schema::placeholder;
use crate::schema::placeholder::dsl::placeholder as all_placeholders;

#[derive(GraphQLObject, Queryable)]
#[graphql(description = "Placeholder object")]
pub struct Placeholder {
  pub id: i32,
  pub text: String,
}

#[derive(GraphQLInputObject, Insertable)]
#[table_name = "placeholder"]
pub struct NewPlaceholder {
  pub text: String,
}

impl Placeholder {
  pub fn get_all(conn: &PoolConn) -> DBResult<Vec<Placeholder>> {
    all_placeholders.load(conn)
  }

  pub fn create(conn: &PoolConn, data: NewPlaceholder) -> DBResult<Placeholder> {
    diesel::insert_into(placeholder::table).values(&data).get_result(conn)
  }
}
