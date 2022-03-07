use crate::schema::thumbnail;
use chrono::NaiveDateTime;
use diesel::MysqlConnection;
use crate::model::customer::Customer;



#[derive(Queryable, Serialize, Deserialize)]
//#[belongs_to(parent = "Customer<'_>")]
#[table_name = "thumbnail"]
pub struct Thumbnail {
    pub id: u32,
    pub asset: Option<String>,
    pub url: Option<u32>,
    pub createdAt: u64,
    pub updatedAt: u64,
}


#[derive(Serialize, Deserialize)]
pub struct ThumbnailList(pub Vec<Post>);

impl ThumbnailList {
    pub fn list(connection: &MysqlConnection) -> Self {
        use crate::schema::thumbnail::dsl::*;
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        let result = thumbnail
            .limit(10)
            .load::<Thumbnail>(connection)
            .expect("Error loading posts");

        ThumbnailList(result)
    }
}
