mod mutations;
mod queries;

use crate::database::schema::campaign_nodes;
use crate::database::schema::campaign_nodes::dsl::*;
use crate::schema::types::RepositoryError::DatabaseError;
use crate::schema::types::{Repository, RepositoryError};
use diesel::{QueryDsl, QueryResult, RunQueryDsl};
pub use mutations::*;
pub use queries::*;
use uuid::Uuid;

impl Repository for DbCampaignNode {
    type Key = Uuid;
    type CreateInput = DbNewCampaignNode;
    type UpdateInput = CampaignNodeInput;

    fn read(pk: Self::Key) -> Result<Option<Self>, RepositoryError> {
        let conn = Self::establish_db_connection();

        let result_nodes = campaign_nodes
            .find(pk)
            .limit(1)
            .load::<DbCampaignNode>(&conn)
            .map_err(|error| DatabaseError(error))?;

        Ok(result_nodes
            .into_iter()
            .next()
            .map_or(None, |node| Some(node)))
    }

    fn read_many(pk: Vec<Self::Key>) -> Result<Vec<(Self::Key, Option<Self>)>, RepositoryError> {
        todo!()
    }

    fn create(input: &Self::CreateInput) -> Result<Self, RepositoryError> {
        let conn = Self::establish_db_connection();

        let result = diesel::insert_into(campaign_nodes::table)
            .values(input)
            .get_result(&conn)
            .map_err(|error| DatabaseError(error))?;

        Ok(result)
    }

    fn update(pk: Self::Key, input: Self::UpdateInput) -> Result<Self, RepositoryError> {
        let conn = Self::establish_db_connection();

        let result = diesel::update(campaign_nodes.find(pk))
            .set(&input)
            .get_result(&conn)
            .map_err(|error| DatabaseError(error))?;

        Ok(result)
    }

    fn delete(pk: Self::Key) -> Result<Self, RepositoryError> {
        let conn = Self::establish_db_connection();

        let result = diesel::delete(campaign_nodes.find(pk))
            .get_result(&conn)
            .map_err(|error| DatabaseError(error))?;

        Ok(result)
    }
}
