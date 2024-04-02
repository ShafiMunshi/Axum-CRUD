use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;


#[derive(Debug, Serialize, Deserialize)]
pub struct Upazila {
    pub name: String,
    pub population: usize,
    pub villages: Vec<String>,
    pub tourist_place: bool,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct Update_Population {
    pub population: usize,
}

#[derive(Debug,Deserialize)]
pub struct UpazilaId{
    pub id: String
}