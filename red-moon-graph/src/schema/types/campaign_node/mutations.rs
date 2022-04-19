use crate::database::schema::campaign_nodes;
use crate::schema::types::campaign_node::DbCampaignNode;
use diesel::query_builder::AsChangeset;
use diesel::AsChangeset;
use juniper::{GraphQLInputObject, ID};
use uuid::Uuid;

#[derive(GraphQLInputObject, AsChangeset)]
#[table_name = "campaign_nodes"]
pub struct CampaignNodeInput {
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}

#[derive(Insertable)]
#[table_name = "campaign_nodes"]
pub struct DbNewCampaignNode {
    pub campaign_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub body: Option<String>,
}

impl From<(ID, CampaignNodeInput)> for DbNewCampaignNode {
    fn from((campaign_id, graph_object): (ID, CampaignNodeInput)) -> Self {
        let CampaignNodeInput {
            title,
            description,
            body,
        } = graph_object;

        DbNewCampaignNode {
            campaign_id: Uuid::parse_str(campaign_id.to_string().as_str()).unwrap(),
            title: title.expect("Title must be present when creating a new campaign node"),
            description,
            body,
        }
    }
}
