mod constants;

mod models;

use std::error::Error;

use reqwest::header::{self, HeaderMap, HeaderValue};

use crate::notion::{
    constants::{BASE_URL, Databases},
    models::{ListUsersResponse, RetrieveDatabaseResponse},
};

use dotenv;

/*
 * Notion Client (currently blocking) to interface with the provided API.
 */
pub struct NotionClient {
    pub http_client: reqwest::blocking::Client, // Leverages persistent connection.
    pub integration_name: String,
    pub token: String,
    pub version: String,
    pub headers: Option<HeaderMap>, // TODO: Implement some sort of caching
}

// TODO: How does this work? The LSP was complaining that I should add this...
impl Default for NotionClient {
    fn default() -> Self {
        Self::new()
    }
}

impl NotionClient {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        // TODO: Should I use expect here?
        Self {
            http_client: reqwest::blocking::Client::new(),
            integration_name: dotenv::var("INTEGRATION_NAME")
                .expect("Please set INTEGRATION_NAME in .env"),
            token: dotenv::var("TOKEN").expect("Please set TOKEN in .env"),
            version: dotenv::var("NOTION_VERSION").expect("Please set NOTION_VERSION in .env"),
            headers: None,
        }
    }

    // TODO: Should I use question mark operators here and propagate the errors?
    pub fn list_users(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let url = String::from(BASE_URL) + "/v1/users";

        let res = self
            .http_client
            .get(url)
            .headers(self.build_headers())
            .send()?;

        let parsed: ListUsersResponse = res.json()?;

        let mut users: Vec<String> = Vec::new();

        for result in parsed.results {
            if result.name != self.integration_name {
                users.push(result.name);
            }
        }

        Ok(users)
    }

    pub fn retrieve_database(
        &self,
        database: Databases,
    ) -> Result<RetrieveDatabaseResponse, Box<dyn Error>> {
        let url = String::from(BASE_URL) + "/v1/databases/" + &database.get_database_id();

        let res = self
            .http_client
            .get(url)
            .headers(self.build_headers())
            .send()?;
        let parsed: RetrieveDatabaseResponse = res.json()?;

        Ok(parsed)
    }

    fn build_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("Notion-Version", HeaderValue::from_static("2025-09-03"));

        let mut bearer_token = String::new();
        bearer_token.push_str("Bearer ");
        bearer_token.push_str(&self.token);

        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&bearer_token).expect("Token contains invalid characters."),
        );
        headers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_users() {
        dotenv::dotenv().ok();
        let notion_client = NotionClient::new();
        assert_eq!(
            vec![dotenv::var("TEST_USER_NAME").expect("Set test user name in .env")],
            notion_client.list_users().unwrap()
        )
    }
}
