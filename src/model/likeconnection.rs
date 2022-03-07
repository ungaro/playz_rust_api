use crate::schema::likeconnection;
use chrono::NaiveDateTime;
use diesel::MysqlConnection;
use crate::model::customer::Customer;



#[derive(Associations,Queryable, Serialize, Deserialize,QueryableByName)]
//#[belongs_to(parent = "Customer<'_>")]
#[table_name = "likeconnection"]
pub struct LikeConnection {
    pub post: String,
    pub who: u32,
    pub createdAt: u64,
    pub updatedAt: u64,
}


#[derive(Serialize, Deserialize)]
pub struct LikeConnectionList(pub Vec<LikeConnection>);

impl LikeConnectionList {
    pub fn list(connection: &MysqlConnection) -> Self {
        use crate::schema::likeconnection::dsl::*;
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        let result = likeconnection
            .limit(10)
            .load::<LikeConnection>(connection)
            .expect("Error loading posts");

            LikeConnectionList(result)
    }
}