use crate::database::schema::campaign_nodes;
use juniper::{GraphQLInputObject, ID};
use uuid::Uuid;

#[derive(GraphQLInputObject)]
pub struct NewCampaignNode {
    pub campaign_id: ID,
    pub title: String,
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

impl From<NewCampaignNode> for DbNewCampaignNode {
    fn from(graph_object: NewCampaignNode) -> Self {
        let NewCampaignNode {
            title,
            description,
            body,
            campaign_id,
        } = graph_object;

        DbNewCampaignNode {
            campaign_id: Uuid::parse_str(campaign_id.to_string().as_str()).unwrap(),
            title,
            description,
            body,
        }
    }
}
