// use crate::diesel::schema::campaign_nodes::dsl::*;
// use diesel::Queryable;

use diesel::prelude::*;
use diesel::Queryable;

#[derive(Queryable)]
pub struct DbCampaign {
    pub id: i32,
}
