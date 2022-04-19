use crate::schema::types::campaign_node::CampaignNode;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use diesel::result::Error;
use diesel::{PgConnection, QueryDsl, Queryable, Table};
use juniper::{graphql_value, FieldError, FieldResult, IntoFieldError, Object, ScalarValue, Value};
use juniper::{EmptySubscription, RootNode};
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};
use serde::Serialize;

pub mod campaign;
pub mod campaign_node;

#[derive(Debug)]
pub struct ValidationError;

#[derive(Debug)]
pub enum RepositoryError {
    DatabaseError(Error),
    ValidationError(Vec<ValidationError>),
}

impl<S: ScalarValue> IntoFieldError<S> for RepositoryError {
    fn into_field_error(self) -> FieldError<S> {
        match self {
            RepositoryError::DatabaseError(error) => {
                FieldError::new(error.to_string(), graphql_value!({"database": "error"}))
            }
            RepositoryError::ValidationError(validation_errors) => {
                let values: Vec<Value> = validation_errors
                    .iter()
                    .map(|error| graphql_value!({}))
                    .collect();

                let mut x: Object<Value> = Object::with_capacity(1);

                x.add_field("fields", Value::List(values));

                FieldError::new("Validation errors", Value::Object(x))
            }
        }
    }
}

pub trait Repository
where
    Self: Sized,
{
    type Key;
    type CreateInput;
    type UpdateInput;

    fn establish_db_connection() -> PgConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url))
    }

    fn read(pk: Self::Key) -> Result<Option<Self>, RepositoryError>;

    fn read_many(pk: Vec<Self::Key>) -> Result<Vec<(Self::Key, Option<Self>)>, RepositoryError>;

    fn create(input: &Self::CreateInput) -> Result<Self, RepositoryError>;

    fn update(pk: Self::Key, input: Self::UpdateInput) -> Result<Self, RepositoryError>;

    fn delete(pk: Self::Key) -> Result<Self, RepositoryError>;
}
