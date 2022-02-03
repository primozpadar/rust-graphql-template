pub mod placeholder;

use diesel::result::Error as DieselError;
use juniper::{FieldError, FieldResult};

// Convert diesel result to GraphQL field result
// more diesel errors: https://docs.rs/diesel/latest/diesel/result/enum.Error.html
pub fn result<T>(res: Result<T, DieselError>) -> FieldResult<T> {
  match res {
    Ok(data) => Ok(data),
    Err(error) => FieldResult::Err(match error {
      DieselError::NotFound => (FieldError::from("Not Found!")),
      _ => FieldError::from("Server Error!"),
    }),
  }
}
