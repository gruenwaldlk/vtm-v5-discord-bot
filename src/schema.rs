table! {
    VTM_ACTIVE_SESSION (session_nr, user_nr, character_nr) {
        session_nr -> Unsigned<Bigint>,
        user_nr -> Unsigned<Bigint>,
        character_nr -> Unsigned<Bigint>,
        character_version_uid -> Unsigned<Bigint>,
    }
}

table! {
    VTM_CHARACTER (character_nr) {
        character_nr -> Unsigned<Bigint>,
        version_uid -> Unsigned<Bigint>,
        json_data -> Json,
        user_nr -> Unsigned<Bigint>,
    }
}

table! {
    VTM_SESSION (session_nr) {
        session_nr -> Unsigned<Bigint>,
        storyteller_nr -> Unsigned<Bigint>,
    }
}

table! {
    VTM_USER (user_nr) {
        user_nr -> Unsigned<Bigint>,
        discord_id -> Unsigned<Bigint>,
    }
}

joinable!(VTM_ACTIVE_SESSION -> VTM_CHARACTER (character_nr));
joinable!(VTM_ACTIVE_SESSION -> VTM_SESSION (session_nr));
joinable!(VTM_ACTIVE_SESSION -> VTM_USER (user_nr));
joinable!(VTM_CHARACTER -> VTM_USER (user_nr));
joinable!(VTM_SESSION -> VTM_USER (storyteller_nr));

allow_tables_to_appear_in_same_query!(
    VTM_ACTIVE_SESSION,
    VTM_CHARACTER,
    VTM_SESSION,
    VTM_USER,
);
