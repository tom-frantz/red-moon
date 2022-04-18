use crate::database::establish_db_connection;
use diesel::prelude::*;
use juniper::FieldResult;

use crate::schema::types::campaign_node::{
    CampaignNode, DbCampaignNode, DbNewCampaignNode, NewCampaignNode,
};
use crate::schema::types::{campaign, campaign_node};

#[juniper::graphql_object]
impl MutationRoot {
    fn create_campaign_node(new_node: NewCampaignNode) -> FieldResult<CampaignNode> {
        use crate::database::schema::campaign_nodes;

        let connection = establish_db_connection();

        let res: DbCampaignNode = diesel::insert_into(campaign_nodes::table)
            .values(&(DbNewCampaignNode::from(new_node)))
            .get_result(&connection)
            .expect("Error saving new post");

        Ok(res.into())
    }
}

pub struct MutationRoot;
