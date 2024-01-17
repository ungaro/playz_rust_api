use crate::schema::gameprofile;
use crate::schema::game;
use chrono::NaiveDateTime;
use diesel::MysqlConnection;
use crate::model::customer::Customer;
 
#[derive(Associations,Queryable, Serialize, Deserialize,QueryableByName,Debug,Clone)]
#[table_name = "game"]
pub struct Game {
    pub id: u32,
    pub slug: Option<String>,
    pub title: String,
    pub icon:Option<String>,
    pub order: Option<i16>,
    pub createdAt: u64,
    pub updatedAt: u64,
}


#[derive(Associations,Queryable, Serialize, Deserialize,QueryableByName,Debug,Clone)]
#[belongs_to(Customer, foreign_key="customer")]
#[belongs_to(Game, foreign_key="game")]
#[table_name = "gameprofile"]
pub struct Gameprofile {
    pub id: u32,
    pub selective_id: String,
    pub game: u32,
    pub customer:u32,
    pub username: String,
    pub region: Option<String>,
    pub platform: Option<String>,
    pub avatar: Option<String>,
    pub uri: Option<String>,
    pub createdAt: u64,
    pub updatedAt: u64,
}


#[derive(Serialize, Deserialize)]
pub struct GameprofileList(pub Vec<Gameprofile>);

impl GameprofileList {
    pub fn list(connection: &MysqlConnection) -> Self {
        use crate::schema::gameprofile::dsl::*;
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        let result = gameprofile
            .limit(10)
            .load::<Gameprofile>(connection)
            .expect("Error loading gameprofiles");

            GameprofileList(result)
    }
}



