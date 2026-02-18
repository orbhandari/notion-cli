use serde::Deserialize;

// TODO: Are these structs good?
#[derive(Deserialize, PartialEq, Debug)]
pub struct ListUsersResponse {
    pub results: Vec<UserMetadata>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct UserMetadata {
    pub name: String,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct RetrieveDatabaseResponse {
    pub object: String,
    pub id: String,
    pub data_sources: Vec<DataSource>,
    pub url: String
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct DataSource {
    pub id: String,
    pub name: String,
}
