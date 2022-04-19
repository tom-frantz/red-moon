use juniper::ID;
use uuid::{Error, Uuid};

pub fn id_to_uuid(id: ID) -> Result<Uuid, Error> {
    Uuid::parse_str(id.to_string().as_str())
}
