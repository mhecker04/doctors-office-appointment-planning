// @generated automatically by Diesel CLI.

diesel::table! {
    user (user_id) {
        #[max_length = 36]
        user_id -> Char,
        #[max_length = 32]
        username -> Nullable<Varchar>,
        #[max_length = 72]
        password -> Nullable<Char>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    user,
);
