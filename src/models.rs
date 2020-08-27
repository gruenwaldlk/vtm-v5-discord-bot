use diesel::Queryable;

#[derive(Queryable)]
pub struct Character {
    character_nr: u64,
    version_uid: u64,
    json_data: String,
    user_nr: u64,
}

#[derive(Queryable)]
pub struct User {
    user_nr: u64,
    discord_id: u64,
}

#[derive(Queryable)]
pub struct Session {
    session_nr: u64,
    storyteller_nr: u64,
}

#[derive(Queryable)]
pub struct ActiveSession{
    session_nr: u64,
    user_nr: u64,
    character_nr: u64,
    character_version_uid: u64,
}
