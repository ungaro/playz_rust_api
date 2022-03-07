use crate::schema::post_asset;
use chrono::NaiveDateTime;
use diesel::MysqlConnection;
use crate::model::customer::Customer;




#[derive(Associations,Queryable, Serialize, Deserialize,QueryableByName,Clone,Debug)]
//#[belongs_to(parent = "Customer<'_>")]
#[table_name = "post_asset"]
pub struct PostAsset {
    pub post: u32,
    pub asset: u32,
    pub order: u32,
    pub url: String,
  


}





#[derive(Serialize, Deserialize)]
pub struct PostAssetList(pub Vec<PostAsset>);

impl PostAssetList {
    pub fn list(connection: &MysqlConnection) -> Self {
        use crate::schema::post_asset::dsl::*;
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        let result = post_asset
            .limit(10)
            .load::<PostAsset>(connection)
            .expect("Error loading posts");

        PostAssetList(result)
    }
}