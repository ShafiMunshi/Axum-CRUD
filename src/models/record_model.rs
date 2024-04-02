use serde::Deserialize;
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(unused)]
    pub id: Thing,
    pub name: String,
    pub population: usize,
    pub villages: Vec<String>,
    pub tourist_place: bool,
}