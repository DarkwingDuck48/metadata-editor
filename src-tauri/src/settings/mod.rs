use crate::database::DatabaseTypes;



/// Settings module for Application
#[derive(Debug, Clone)]
pub struct Settings {

    database_type: DatabaseTypes,
    database_url: String
}