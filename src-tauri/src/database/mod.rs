/// Module for database connections for application and work with metadata


mod sqlite_connector;

#[derive(Debug, Clone)]
pub enum DatabaseTypes {
    SQLite
}

struct DatabaseConnector{
    
}
