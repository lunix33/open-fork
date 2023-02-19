// @generated automatically by Diesel CLI.

diesel::table! {
    recipes (id) {
        id -> Text,
        name -> Text,
        image -> Nullable<Binary>,
        created_on -> Timestamp,
        updated_on -> Timestamp,
    }
}
