use crate::database::establish_db_connection;
use diesel::prelude::*;
use juniper::{graphql_value, FieldError, FieldResult, ID};

use crate::schema::types::campaign_node::{
    CampaignNode, CampaignNodeInput, DbCampaignNode, DbNewCampaignNode,
};
use crate::schema::types::Repository;
use crate::schema::types::{campaign, campaign_node};
use crate::utils::id_to_uuid;

#[juniper::graphql_object]
impl MutationRoot {
    fn campaign_node_create(
        _campaign_id: ID,
        input: CampaignNodeInput,
    ) -> FieldResult<CampaignNode> {
        if let None = input.title {
            return Err(FieldError::new(
                "Cannot create a new campaign node. Title missing",
                graphql_value!({"fields": [{"name": "title", "error": "missing"}]}),
            ));
        }

        let db_input = DbNewCampaignNode::from((_campaign_id, input));

        DbCampaignNode::create(&db_input)?.into()
    }

    fn campaign_node_update(_id: ID, input: CampaignNodeInput) -> FieldResult<CampaignNode> {
        use crate::database::schema::campaign_nodes::dsl::*;

        let connection = establish_db_connection();
        let uuid = id_to_uuid(_id.clone()).unwrap();
        let update_object: DbCampaignNode = diesel::update(campaign_nodes.find(uuid))
            .set(&input)
            .get_result(&connection)?;

        Ok(update_object.into())
    }

    // fn campaign_node_delete(_id: ID) -> FieldResult<CampaignNode> {}
}

pub struct MutationRoot;
