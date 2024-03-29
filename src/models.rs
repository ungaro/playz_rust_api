// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]



#[derive(Queryable, Debug, Identifiable)]
pub struct Customer {
    pub id: /* TODO: unknown type Unsigned<Integer> */,
    pub displayName: String,
    pub info: Option</* TODO: unknown type Nullable<Unsigned<Integer>> */>,
    pub active: Option</* TODO: unknown type Nullable<Unsigned<Tinyint>> */>,
    pub authLevel: Option</* TODO: unknown type Nullable<Unsigned<Smallint>> */>,
    pub banned: Option</* TODO: unknown type Nullable<Unsigned<Tinyint>> */>,
    pub apnFollowEnabled: Option</* TODO: unknown type Nullable<Unsigned<Tinyint>> */>,
    pub apnRepostEnabled: Option</* TODO: unknown type Nullable<Unsigned<Tinyint>> */>,
    pub apnCommentEnabled: Option</* TODO: unknown type Nullable<Unsigned<Tinyint>> */>,
    pub apnLikeEnabled: Option</* TODO: unknown type Nullable<Unsigned<Tinyint>> */>,
    pub apnMessageEnabled: Option</* TODO: unknown type Nullable<Unsigned<Tinyint>> */>,
    pub apnInfoEnabled: Option</* TODO: unknown type Nullable<Unsigned<Tinyint>> */>,
    pub apnMentionEnabled: Option</* TODO: unknown type Nullable<Unsigned<Tinyint>> */>,
    pub apnEncodeEnabled: Option</* TODO: unknown type Nullable<Unsigned<Tinyint>> */>,
    pub apnMatchEnabled: Option</* TODO: unknown type Nullable<Unsigned<Tinyint>> */>,
    pub apnPanelInviteEnabled: /* TODO: unknown type Unsigned<Tinyint> */,
    pub apnEnabled: Option</* TODO: unknown type Nullable<Unsigned<Tinyint>> */>,
    pub followerCount: Option</* TODO: unknown type Nullable<Unsigned<Integer>> */>,
    pub followingCount: Option</* TODO: unknown type Nullable<Unsigned<Integer>> */>,
    pub mentionCount: Option</* TODO: unknown type Nullable<Unsigned<Integer>> */>,
    pub mentionedCount: Option</* TODO: unknown type Nullable<Unsigned<Integer>> */>,
    pub notificationCount: Option</* TODO: unknown type Nullable<Unsigned<Integer>> */>,
    pub postCount: Option</* TODO: unknown type Nullable<Unsigned<Integer>> */>,
    pub likeCount: Option</* TODO: unknown type Nullable<Unsigned<Integer>> */>,
    pub commentCount: Option</* TODO: unknown type Nullable<Unsigned<Integer>> */>,
    pub tagCount: Option</* TODO: unknown type Nullable<Unsigned<Integer>> */>,
    pub displayName: String,
    pub lastOnline: Option</* TODO: unknown type Nullable<Unsigned<Bigint>> */>,
    pub gsm: Option<String>,
    pub sessionId: Option<String>,
    pub isFamous: Option</* TODO: unknown type Nullable<Unsigned<Tinyint>> */>,
    pub createdAt: Option</* TODO: unknown type Nullable<Unsigned<Bigint>> */>,
    pub updatedAt: Option</* TODO: unknown type Nullable<Unsigned<Bigint>> */>,
    pub lastIp: Option<String>,
    pub isVerified: Option<bool>,
    pub anonymUser: Option</* TODO: unknown type Nullable<Smallint> */>,
    pub taskLevel: Option</* TODO: unknown type Nullable<Smallint> */>,
    pub firebaseToken: Option<String>,
}

#[derive(Queryable, Debug, Identifiable)]
pub struct Customerinformation {
    pub id: /* TODO: unknown type Unsigned<Integer> */,
    pub customer: /* TODO: unknown type Unsigned<Integer> */,
    pub photo: Option<String>,
    pub address: Option<String>,
    pub name: Option</* TODO: unknown type Nullable<Tinytext> */>,
    pub surname: Option</* TODO: unknown type Nullable<Tinytext> */>,
    pub birthday: Option</* TODO: unknown type Nullable<Double> */>,
    pub gender: Option<String>,
    pub biography: Option<String>,
    pub county: Option</* TODO: unknown type Nullable<Unsigned<Integer>> */>,
    pub city: Option</* TODO: unknown type Nullable<Unsigned<Integer>> */>,
    pub createdAt: /* TODO: unknown type Unsigned<Bigint> */,
    pub updatedAt: /* TODO: unknown type Unsigned<Bigint> */,
    pub refcode: Option</* TODO: unknown type Nullable<Char> */>,
    pub registerRefCode: Option</* TODO: unknown type Nullable<Char> */>,
    pub taskGroupId: /* TODO: unknown type Mediumint */,
    pub missionLevel: Option</* TODO: unknown type Nullable<Tinyint> */>,
    pub missionSubLevel: Option</* TODO: unknown type Nullable<Tinyint> */>,
    pub missionLevelString: Option<String>,
}

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(blocker, blocked)]
pub struct Blockconnection {
    pub blocker: /* TODO: unknown type Unsigned<Integer> */,
    pub blocked: /* TODO: unknown type Unsigned<Integer> */,
    pub createdAt: /* TODO: unknown type Unsigned<Bigint> */,
    pub updatedAt: /* TODO: unknown type Unsigned<Bigint> */,
}

#[derive(Queryable, Debug, Identifiable)]
pub struct Gameprofile {
    pub id: /* TODO: unknown type Unsigned<Integer> */,
    pub selective_id: String,
    pub game: /* TODO: unknown type Unsigned<Integer> */,
    pub customer: /* TODO: unknown type Unsigned<Integer> */,
    pub username: String,
    pub region: Option<String>,
    pub platform: Option<String>,
    pub avatar: Option<String>,
    pub uri: Option<String>,
    pub createdAt: Option</* TODO: unknown type Nullable<Bigint> */>,
    pub updatedAt: Option</* TODO: unknown type Nullable<Bigint> */>,
}

#[derive(Queryable, Debug, Identifiable)]
pub struct Customerinformation {
    pub id: /* TODO: unknown type Unsigned<Integer> */,
    pub customer: /* TODO: unknown type Unsigned<Integer> */,
    pub photo: Option<String>,
    pub address: Option<String>,
    pub name: Option</* TODO: unknown type Nullable<Tinytext> */>,
    pub surname: Option</* TODO: unknown type Nullable<Tinytext> */>,
    pub birthday: Option</* TODO: unknown type Nullable<Double> */>,
    pub gender: Option<String>,
    pub biography: Option<String>,
    pub county: Option</* TODO: unknown type Nullable<Unsigned<Integer>> */>,
    pub city: Option</* TODO: unknown type Nullable<Unsigned<Integer>> */>,
    pub createdAt: /* TODO: unknown type Unsigned<Bigint> */,
    pub updatedAt: /* TODO: unknown type Unsigned<Bigint> */,
    pub refcode: Option</* TODO: unknown type Nullable<Char> */>,
    pub registerRefCode: Option</* TODO: unknown type Nullable<Char> */>,
    pub taskGroupId: i32,
    pub missionLevel: Option</* TODO: unknown type Nullable<Tinyint> */>,
    pub missionSubLevel: Option</* TODO: unknown type Nullable<Tinyint> */>,
    pub missionLevelString: Option<String>,
}

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(post, who)]
pub struct Likeconnection {
    pub post: String,
    pub who: /* TODO: unknown type Unsigned<Integer> */,
    pub //type_: /* TODO: unknown type Enum */,
    pub createdAt: /* TODO: unknown type Unsigned<Bigint> */,
    pub updatedAt: /* TODO: unknown type Unsigned<Bigint> */,
}

#[derive(Queryable, Debug, Identifiable)]
pub struct Mentionconnection {
    pub id: String,
    pub who: Option</* TODO: unknown type Nullable<Unsigned<Integer>> */>,
    pub with: Option</* TODO: unknown type Nullable<Unsigned<Integer>> */>,
    pub startIndex: /* TODO: unknown type Unsigned<Integer> */,
    pub text: Option<String>,
    pub creator: Option</* TODO: unknown type Nullable<Unsigned<Integer>> */>,
    pub //type_: Option</* TODO: unknown type Nullable<Enum> */>,
    pub createdAt: /* TODO: unknown type Unsigned<Bigint> */,
    pub updatedAt: /* TODO: unknown type Unsigned<Bigint> */,
}

#[derive(Queryable, Debug, Identifiable)]
pub struct Post {
    pub id: /* TODO: unknown type Unsigned<Integer> */,
    pub text: Option<String>,
    pub survey: Option</* TODO: unknown type Nullable<Unsigned<Integer>> */>,
    pub isReady: Option</* TODO: unknown type Nullable<Unsigned<Tinyint>> */>,
    pub pinned: Option</* TODO: unknown type Nullable<Unsigned<Tinyint>> */>,
    pub weight: Option</* TODO: unknown type Nullable<Unsigned<Integer>> */>,
    pub original: Option</* TODO: unknown type Nullable<Unsigned<Integer>> */>,
    pub section: Option</* TODO: unknown type Nullable<Unsigned<Integer>> */>,
    pub createdByPlayz: Option</* TODO: unknown type Nullable<Unsigned<Tinyint>> */>,
    pub customer: Option</* TODO: unknown type Nullable<Unsigned<Integer>> */>,
    pub viewCount: Option</* TODO: unknown type Nullable<Unsigned<Integer>> */>,
    pub likeCount: Option</* TODO: unknown type Nullable<Unsigned<Integer>> */>,
    pub reportCount: Option</* TODO: unknown type Nullable<Unsigned<Integer>> */>,
    pub mentionCount: Option</* TODO: unknown type Nullable<Unsigned<Integer>> */>,
    pub tagCount: Option</* TODO: unknown type Nullable<Unsigned<Integer>> */>,
    pub likeRange: Option</* TODO: unknown type Nullable<Unsigned<Integer>> */>,
    pub commentCount: Option</* TODO: unknown type Nullable<Unsigned<Integer>> */>,
    pub createdAt: /* TODO: unknown type Unsigned<Bigint> */,
    pub updatedAt: /* TODO: unknown type Unsigned<Bigint> */,
}

#[derive(Queryable, Debug, Identifiable)]
pub struct Asset {
    pub id: /* TODO: unknown type Unsigned<Integer> */,
    pub customer: Option</* TODO: unknown type Nullable<Unsigned<Integer>> */>,
    pub type_: String,
    pub path: Option<String>,
    pub url: Option<String>,
    pub width: Option</* TODO: unknown type Nullable<Unsigned<Integer>> */>,
    pub height: Option</* TODO: unknown type Nullable<Unsigned<Integer>> */>,
    pub name: Option<String>,
    pub format: Option<String>,
    pub duration: Option</* TODO: unknown type Nullable<Unsigned<Integer>> */>,
    pub frameCount: Option</* TODO: unknown type Nullable<Unsigned<Bigint>> */>,
    pub frameRate: Option</* TODO: unknown type Nullable<Unsigned<Bigint>> */>,
    pub extension: Option<String>,
    pub size: Option</* TODO: unknown type Nullable<Unsigned<Bigint>> */>,
    pub isDeleted: /* TODO: unknown type Unsigned<Tinyint> */,
    pub isReady: Option</* TODO: unknown type Nullable<Unsigned<Tinyint>> */>,
    pub isFailed: /* TODO: unknown type Unsigned<Tinyint> */,
    pub updatedAt: /* TODO: unknown type Unsigned<Bigint> */,
    pub createdAt: /* TODO: unknown type Unsigned<Bigint> */,
}

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(post, asset)]
pub struct PostAsset {
    pub post: /* TODO: unknown type Unsigned<Integer> */,
    pub asset: /* TODO: unknown type Unsigned<Integer> */,
    pub order: /* TODO: unknown type Unsigned<Integer> */,
}

#[derive(Queryable, Debug{trace1})]
pub struct Thumbnail {
    pub id: /* TODO: unknown type Unsigned<Integer> */,
    pub asset: /* TODO: unknown type Unsigned<Integer> */,
    pub url: String,
    pub createdAt: /* TODO: unknown type Unsigned<Bigint> */,
    pub updatedAt: /* TODO: unknown type Unsigned<Bigint> */,
}

