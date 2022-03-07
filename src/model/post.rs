use crate::schema::post;
use chrono::NaiveDateTime;
use diesel::MysqlConnection;
use crate::model::customer::Customer;



pub enum PostType {
    P
}



#[derive(Associations,Queryable, Serialize, Deserialize,QueryableByName,Clone,Debug)]
//#[belongs_to(parent = "Customer<'_>")]
#[belongs_to(Customer, foreign_key="customer")]
#[table_name = "post"]
pub struct Post {
    pub id: u32,
    pub text: Option<String>,
    pub survey: Option<u32>,
    pub isReady: Option<u8>,
    pub pinned: Option<u8>,
    pub weight: Option<u32>,
    pub original: Option<u32>,
    pub section: Option<u32>,
    pub createdByPlayz: Option<u8>,
    pub customer: Option<u32>,
    pub viewCount: Option<u32>,
    pub likeCount: Option<u32>,
    pub reportCount: Option<u32>,
    pub mentionCount: Option<u32>,
    pub tagCount: Option<u32>,
    pub likeRange: Option<u32>,
    pub commentCount: Option<u32>,
    pub createdAt: u64,
    pub updatedAt: u64,


}


#[derive(Serialize, Deserialize)]
pub struct PostList(pub Vec<Post>);

impl PostList {
    pub fn list(connection: &MysqlConnection) -> Self {
        use crate::schema::post::dsl::*;
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        let result = post
            .limit(10)
            .load::<Post>(connection)
            .expect("Error loading posts");

        PostList(result)
    }
}