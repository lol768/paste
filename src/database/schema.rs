table! {
    api_keys (key) {
        key -> Uuid,
        user_id -> Uuid,
    }
}

table! {
    deletion_keys (key) {
        key -> Uuid,
        paste_id -> Uuid,
    }
}

table! {
    files (id) {
        id -> Uuid,
        paste_id -> Uuid,
        name -> Nullable<Text>,
        is_binary -> Nullable<Bool>,
    }
}

table! {
    pastes (id) {
        id -> Uuid,
        name -> Nullable<Text>,
        visibility -> Int2,
        author_id -> Nullable<Uuid>,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        password -> Text,
    }
}

joinable!(api_keys -> users (user_id));
joinable!(deletion_keys -> pastes (paste_id));
joinable!(files -> pastes (paste_id));
joinable!(pastes -> users (author_id));

allow_tables_to_appear_in_same_query!(
    api_keys,
    deletion_keys,
    files,
    pastes,
    users,
);