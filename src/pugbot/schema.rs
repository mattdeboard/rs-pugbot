table! {
    user_ratings (id) {
        id -> Int4,
        user_id -> Int4,
        rating -> Numeric,
    }
}

table! {
    users (user_id) {
        user_id -> Int4,
        bot -> Bool,
        discriminator -> Varchar,
        name -> Varchar,
    }
}
