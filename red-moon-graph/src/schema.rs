use juniper::FieldResult;
use juniper::{EmptySubscription, RootNode};
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

use mutation::MutationRoot;
use query::QueryRoot;

mod mutation;
mod query;

pub mod types;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;
pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
