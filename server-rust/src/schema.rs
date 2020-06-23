table! {
    posts (id) {
        id -> Int4,
        title -> Nullable<Varchar>,
        img -> Nullable<Text>,
        content -> Nullable<Text>,
        author_id -> Nullable<Int4>,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Text,
        profile_pic -> Nullable<Text>,
    }
}

joinable!(posts -> users (author_id));

allow_tables_to_appear_in_same_query!(posts, users,);
