use serde::Deserialize;

#[derive(Deserialize)]
pub struct Env {
    pub discord_token: String,
    pub discord_application_id: u64,
    pub discord_guild_id: u64,
}

impl Env {
    pub fn new() -> Self {
        envy::from_env::<Self>().expect("必要な環境変数を取得できませんでした。")
    }
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}
