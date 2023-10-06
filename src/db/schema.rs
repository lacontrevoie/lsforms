diesel::table! {
    transactions (id) {
        id -> Integer,
        username -> Nullable<Text>,
        message -> Nullable<Text>,
        email -> Text,
        time -> Timestamp,
        amount -> Integer,
        gems -> Integer,
        token -> Text,
        is_mail_sent -> Bool,
        is_token_used -> Bool,
    }
}

diesel::table! {
    stars (id) {
        id -> Integer,
        startype -> Integer,
        position_x -> Float,
        position_y -> Float,
        transactionid -> Integer,
    }
}

diesel::joinable!(stars -> transactions (transactionid));

diesel::allow_tables_to_appear_in_same_query!(
    stars,
    transactions,
);
