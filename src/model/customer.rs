use crate::schema::customer;
use crate::schema::customerinformation;
use chrono::NaiveDateTime;
use diesel::MysqlConnection;


#[derive(Associations,Queryable, Serialize, Deserialize,QueryableByName,Clone,Debug)]
#[table_name = "customerinformation"]
pub struct CustomerInfo{
    pub id: u32,
    pub customer: u32,
    pub photo: String,
    pub address: Option<String>,
    pub name: String,
    pub surname: String,
    pub birthday: u64,
    pub gender: Option<String>,
    pub biography: u32,
    pub county: Option<String>,
    pub city: Option<String>,
    pub createdAt: u64,
    pub updatedAt: u64,
    pub refcode: String,
    pub registerRefCode: String,
    pub taskGroupId: u32,
    pub missionLevel: u32,
    pub missionSubLevel: u32,
    pub missionLevelString: String,

}





#[derive(Associations,Queryable, Serialize, Deserialize,QueryableByName,Clone,Debug)]
#[table_name = "customer"]
pub struct Customer {
    pub id: u32,
    pub active: Option<u8>,
    pub displayName: String,
    //pub info: String,

    pub authLevel: Option<u16>,
    pub banned: Option<u8>,
    pub apnFollowEnabled: Option<u8>,
    pub apnRepostEnabled: Option<u8>,
    pub apnCommentEnabled: Option<u8>,
    pub apnLikeEnabled: Option<u8>,
    pub apnMessageEnabled: Option<u8>,
    pub apnInfoEnabled: Option<u8>,
    pub apnMentionEnabled: Option<u8>,
    pub apnEncodeEnabled: Option<u8>,
    pub apnMatchEnabled: Option<u8>,
    pub apnPanelInviteEnabled: u8,
    pub apnEnabled: Option<u8>,
 

    pub followerCount: Option<u32>,
    pub followingCount: Option<u32>,
    pub mentionCount: Option<u32>,
    pub mentionedCount: Option<u32>,
    pub notificationCount: Option<u32>,
    pub postCount: Option<u32>,
    pub likeCount: Option<u32>,
    pub commentCount: Option<u32>,


    pub tagCount: Option<u32>,
    pub lastOnline: Option<u64>,
    pub gsm: Option<String>,
    pub sessionId: Option<String>,
    pub isFamous: Option<u8>,
    pub createdAt: Option<u64>,
    pub updatedAt: Option<u64>,
    pub lastIp: Option<String>,
    pub isVerified: Option<bool>,
    pub anonymUser: Option<i16>,
    pub taskLevel: Option<i16>,
    pub firebaseToken: Option<String>,
    //pub info: Option<CustomerInfo>,





    
    /*
    pub lastOnline: String,
    pub gsm: String,
    pub sessionId: String,
    pub isFamous: String,
    pub info: CustomerInfo,

    info -> Nullable<Unsigned<Integer>>,
        active -> Nullable<Unsigned<Tinyint>>,
        authLevel -> Nullable<Unsigned<Smallint>>,
        banned -> Nullable<Unsigned<Tinyint>>,
        apnFollowEnabled -> Nullable<Unsigned<Tinyint>>,
        apnRepostEnabled -> Nullable<Unsigned<Tinyint>>,
        apnCommentEnabled -> Nullable<Unsigned<Tinyint>>,
        apnLikeEnabled -> Nullable<Unsigned<Tinyint>>,
        apnMessageEnabled -> Nullable<Unsigned<Tinyint>>,
        apnInfoEnabled -> Nullable<Unsigned<Tinyint>>,
        apnMentionEnabled -> Nullable<Unsigned<Tinyint>>,
        apnEncodeEnabled -> Nullable<Unsigned<Tinyint>>,
        apnMatchEnabled -> Nullable<Unsigned<Tinyint>>,
        apnPanelInviteEnabled -> Unsigned<Tinyint>,
        apnEnabled -> Nullable<Unsigned<Tinyint>>,
        followerCount -> Nullable<Unsigned<Integer>>,
        followingCount -> Nullable<Unsigned<Integer>>,
        mentionCount -> Nullable<Unsigned<Integer>>,
        mentionedCount -> Nullable<Unsigned<Integer>>,
        notificationCount -> Nullable<Unsigned<Integer>>,
        postCount -> Nullable<Unsigned<Integer>>,
        likeCount -> Nullable<Unsigned<Integer>>,
        commentCount -> Nullable<Unsigned<Integer>>,
        tagCount -> Nullable<Unsigned<Integer>>,
        displayName -> Varchar,
        lastOnline -> Nullable<Unsigned<Bigint>>,
        gsm -> Nullable<Varchar>,
        sessionId -> Nullable<Varchar>,
        isFamous -> Nullable<Unsigned<Tinyint>>,
        createdAt -> Nullable<Unsigned<Bigint>>,
        updatedAt -> Nullable<Unsigned<Bigint>>,
        lastIp -> Nullable<Varchar>,
        isVerified -> Nullable<Bool>,
        anonymUser -> Nullable<Smallint>,
        taskLevel -> Nullable<Smallint>,
        firebaseToken -> Nullable<Varchar>,

*/

}

#[derive(Serialize, Deserialize)]
pub struct CustomerList(pub Vec<Customer>);

impl CustomerList {
    pub fn list(connection: &MysqlConnection) -> Self {
        use crate::schema::customer::dsl::*;
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        let result = customer
            .limit(50)
            .load::<Customer>(connection)
            .expect("Error loading users");

        CustomerList(result)
    }
}