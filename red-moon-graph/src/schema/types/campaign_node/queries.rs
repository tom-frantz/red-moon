use crate::database::schema::campaign_nodes;
use diesel::AsChangeset;
use juniper::GraphQLObject;
use juniper::ID;
use uuid::Uuid;

#[derive(GraphQLObject)]
#[graphql(description = "A Node as a part of a campaign")]
pub struct CampaignNode {
    pub id: ID,
    pub campaign_id: ID,
    pub title: String,
    pub description: Option<String>,
    pub body: Option<String>,
}

#[derive(Queryable, AsChangeset, Debug)]
#[table_name = "campaign_nodes"]
pub struct DbCampaignNode {
    pub id: Uuid,
    pub campaign_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub body: Option<String>,
}

impl From<DbCampaignNode> for CampaignNode {
    fn from(model: DbCampaignNode) -> Self {
        let DbCampaignNode {
            id,
            campaign_id: _campaign_id,
            title,
            description,
            body,
        } = model;

        CampaignNode {
            id: ID::new(id.to_string()),
            campaign_id: ID::new(_campaign_id.to_string()),
            title,
            description,
            body,
        }
    }
}
