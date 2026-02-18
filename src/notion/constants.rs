use dotenv::dotenv;

pub const BASE_URL: &str = "https://api.notion.com";

// Only allow a fixed subset of databases for the moment.
// TODO: Use search API instead.
pub enum Databases {
    KnowledgeHub,
    TestDatabase,
}

impl Databases {
    pub fn as_str(&self) -> &'static str {
        match self {
            Databases::KnowledgeHub => "Knowledge Hub",
            Databases::TestDatabase => "Test Database",
        }
    }

    pub fn get_database_id(&self) -> String {
        dotenv().ok();
        match self {
            Databases::KnowledgeHub => {
                dotenv::var("KNOWLEDGE_HUB_DATABASE_ID").expect("Please set database ID in .env")
            }
            Databases::TestDatabase => {
                dotenv::var("TEST_DATABASE_ID").expect("Please set database ID in .env")
            }
        }
    }
}
