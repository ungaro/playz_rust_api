table! {
    customer (id) {
        id -> Unsigned<Integer>,
        //info -> Nullable<Unsigned<Integer>>,
        active -> Nullable<Unsigned<Tinyint>>,
        displayName -> Varchar,
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
         /* */
     
    }
}

table! {
    customerinformation (id) {
        id -> Unsigned<Integer>,
        customer -> Unsigned<Integer>,
        photo -> Nullable<Varchar>,
        address -> Nullable<Varchar>,
        name -> Nullable<Tinytext>,
        surname -> Nullable<Tinytext>,
        birthday -> Nullable<Double>,
        gender -> Nullable<Varchar>,
        biography -> Nullable<Varchar>,
        county -> Nullable<Unsigned<Integer>>,
        city -> Nullable<Unsigned<Integer>>,
        createdAt -> Unsigned<Bigint>,
        updatedAt -> Unsigned<Bigint>,
        refcode -> Nullable<Char>,
        registerRefCode -> Nullable<Char>,
        taskGroupId -> Nullable<Integer>,
        missionLevel -> Nullable<Tinyint>,
        missionSubLevel -> Nullable<Tinyint>,
        missionLevelString -> Nullable<Varchar>,
    }
}



table! {
    blockconnection (blocker, blocked) {
        blocker -> Unsigned<Integer>,
        blocked -> Unsigned<Integer>,
        createdAt -> Unsigned<Bigint>,
        updatedAt -> Unsigned<Bigint>,
    }
}

table! {
    gameprofile (id) {
        id -> Unsigned<Integer>,
        selective_id -> Varchar,
        game -> Unsigned<Integer>,
        customer -> Unsigned<Integer>,
        username -> Varchar,
        region -> Nullable<Varchar>,
        platform -> Nullable<Varchar>,
        avatar -> Nullable<Varchar>,
        uri -> Nullable<Varchar>,
        createdAt -> Unsigned<Bigint>,
        updatedAt -> Unsigned<Bigint>,
    }
}

table! {
    game (id) {
        id -> Unsigned<Integer>,
        slug -> Nullable<Varchar>,
        title -> Varchar,
        icon -> Nullable<Varchar>,
        order -> Nullable<Smallint>,
        createdAt -> Unsigned<Bigint>,
        updatedAt -> Unsigned<Bigint>,
    }
}



/*
table! {
    customerinformation (id) {
        id -> Unsigned<Integer>,
        customer -> Unsigned<Integer>,
        photo -> Nullable<Varchar>,
        address -> Nullable<Varchar>,
        name -> Nullable<Tinytext>,
        surname -> Nullable<Tinytext>,
        birthday -> Nullable<Double>,
        gender -> Nullable<Varchar>,
        biography -> Nullable<Varchar>,
        county -> Nullable<Unsigned<Integer>>,
        city -> Nullable<Unsigned<Integer>>,
        createdAt -> Unsigned<Bigint>,
        updatedAt -> Unsigned<Bigint>,
        refcode -> Nullable<Char>,
        registerRefCode -> Nullable<Char>,
        taskGroupId -> Integer,
        missionLevel -> Nullable<Tinyint>,
        missionSubLevel -> Nullable<Tinyint>,
        missionLevelString -> Nullable<Varchar>,
    }
}
*/


table! {
    likeconnection (post, who) {
        post -> Varchar,
        who -> Unsigned<Integer>,
        //#[sql_name = "type"]
        //type_ -> Enum,
        createdAt -> Unsigned<Bigint>,
        updatedAt -> Unsigned<Bigint>,
    }
}

table! {
    mentionconnection (id) {
        id -> Varchar,
        who -> Nullable<Unsigned<Integer>>,
        with -> Nullable<Unsigned<Integer>>,
        startIndex -> Unsigned<Integer>,
        text -> Nullable<Varchar>,
        creator -> Nullable<Unsigned<Integer>>,
        //#[sql_name = "type"]
        //type_ -> Nullable<Enum>,
        createdAt -> Unsigned<Bigint>,
        updatedAt -> Unsigned<Bigint>,
    }
}


table! {
    post (id) {
        id -> Unsigned<Integer>,
        text -> Nullable<Text>,
        survey -> Nullable<Unsigned<Integer>>,
        isReady -> Nullable<Unsigned<Tinyint>>,
        pinned -> Nullable<Unsigned<Tinyint>>,
        weight -> Nullable<Unsigned<Integer>>,
        original -> Nullable<Unsigned<Integer>>,
        section -> Nullable<Unsigned<Integer>>,
        createdByPlayz -> Nullable<Unsigned<Tinyint>>,
        customer -> Nullable<Unsigned<Integer>>,
        viewCount -> Nullable<Unsigned<Integer>>,
        likeCount -> Nullable<Unsigned<Integer>>,
        reportCount -> Nullable<Unsigned<Integer>>,
        mentionCount -> Nullable<Unsigned<Integer>>,
        tagCount -> Nullable<Unsigned<Integer>>,
        likeRange -> Nullable<Unsigned<Integer>>,
        commentCount -> Nullable<Unsigned<Integer>>,
        createdAt -> Unsigned<Bigint>,
        updatedAt -> Unsigned<Bigint>,
    }
}


table! {
    asset (id) {
        id -> Unsigned<Integer>,
        customer -> Nullable<Unsigned<Integer>>,
        #[sql_name = "type"]
        type_ -> Varchar,
        path -> Nullable<Varchar>,
        url -> Nullable<Varchar>,
        width -> Nullable<Unsigned<Integer>>,
        height -> Nullable<Unsigned<Integer>>,
        name -> Nullable<Varchar>,
        format -> Nullable<Varchar>,
        duration -> Nullable<Unsigned<Integer>>,
        frameCount -> Nullable<Unsigned<Bigint>>,
        frameRate -> Nullable<Unsigned<Bigint>>,
        extension -> Nullable<Varchar>,
        size -> Nullable<Unsigned<Bigint>>,
        isDeleted -> Unsigned<Tinyint>,
        isReady -> Nullable<Unsigned<Tinyint>>,
        isFailed -> Unsigned<Tinyint>,
        updatedAt -> Unsigned<Bigint>,
        createdAt -> Unsigned<Bigint>,
    }
}

table! {
    post_asset (post, asset) {
        post -> Unsigned<Integer>,
        asset -> Unsigned<Integer>,
        order -> Unsigned<Integer>,
        url -> Varchar,
    }
}


table! {
    thumbnail (id) {
        id -> Unsigned<Integer>,
        asset -> Unsigned<Integer>,
        url -> Varchar,
        createdAt -> Unsigned<Bigint>,
        updatedAt -> Unsigned<Bigint>,
    }
}



//joinable!(thumbnail -> post_asset (asset));



allow_tables_to_appear_in_same_query!(
    customer,
    post_asset,
    thumbnail,
    customerinformation,
);
