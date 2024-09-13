use serde::Serialize;

#[derive(Serialize)]
pub struct UKey {
    pub id: i32,
    pub ukey: String,
    pub exp_date: Option<chrono::NaiveDateTime>,
    pub last_used: Option<chrono::NaiveDateTime>,
    pub n_used: i32,
    pub revoked: bool
}