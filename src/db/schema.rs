diesel::table! {
    transaction (id) {
        id -> Integer,
        username -> Text,
        message -> Text,
        email -> Text,
        day -> Date,
        amount -> Integer,
        gems -> Integer,
        token -> Text,
        ha_id -> Integer,
        receipt_url -> Text,
        event_type -> Integer,
        is_mail_sent -> Bool,
        is_token_used -> Bool,
        is_checked -> Bool,
    }
}

diesel::table! {
    star (id) {
        id -> Integer,
        startype -> Integer,
        position_x -> Float,
        position_y -> Float,
        transactionid -> Integer,
    }
}

diesel::joinable!(star -> transaction (transactionid));

diesel::allow_tables_to_appear_in_same_query!(
    star,
    transaction,
);
