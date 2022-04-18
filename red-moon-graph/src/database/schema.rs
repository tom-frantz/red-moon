table! {
    campaign_nodes (id) {
        id -> Uuid,
        campaign_id -> Uuid,
        title -> Text,
        description -> Nullable<Text>,
        body -> Nullable<Text>,
    }
}

table! {
    campaigns (id) {
        id -> Uuid,
    }
}

joinable!(campaign_nodes -> campaigns (campaign_id));

allow_tables_to_appear_in_same_query!(
    campaign_nodes,
    campaigns,
);
