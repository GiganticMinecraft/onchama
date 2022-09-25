use crate::command::{CommandResult, Context};

/// 読み上げを開始します
#[poise::command(slash_command)]
pub async fn connect(ctx: Context<'_>) -> CommandResult {
    Ok(())
}
