-- Your SQL goes here
WITH campaign AS (INSERT INTO campaigns DEFAULT VALUES RETURNING id)
    INSERT INTO campaign_nodes (campaign_id, title, description, body) VALUES (
        (SELECT (campaign.id) FROM campaign),
        'Node Title',
        'This is a node description',
        '<p>This is a node body</p>'
    )
