use crate::database::establish_db_connection;
use crate::schema::types::campaign_node::{CampaignNode, DbCampaignNode};
use juniper::{FieldError, FieldResult, ID};
use uuid::Uuid;

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn campaign_node(_id: String) -> FieldResult<Option<CampaignNode>> {
        use crate::database::schema::campaign_nodes::dsl::*;
        use diesel::prelude::*;

        let connection = establish_db_connection();

        let db_nodes = campaign_nodes
            // TODO: Do something about this error.
            .find(Uuid::parse_str(&_id)?)
            .limit(1)
            .load::<DbCampaignNode>(&connection)
            .expect("Error loading campaign nodes.");

        let graph_node_result = db_nodes
            .into_iter()
            .next()
            .map_or(None, |node| Some(node.into()));

        Ok(graph_node_result)
    }
}
