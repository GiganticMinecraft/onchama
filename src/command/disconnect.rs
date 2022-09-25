use crate::{
    command::{CommandResult, Context},
    shared::CreateDiscordEmbedExt,
};

use anyhow::{ensure, Context as _};

/// 読み上げを終了します
#[poise::command(slash_command)]
pub async fn disconnect(ctx: Context<'_>) -> CommandResult {
    let guild = ctx.guild().context("Guildがありません")?;
    let guild_id = guild.id;
    let manager = songbird::get(&ctx.discord())
        .await
        .context("Songbirdが初期化されていません")?;

    ensure!(manager.get(guild_id).is_some(), "BotはVCに参加していません");

    manager
        .remove(guild_id)
        .await
        .context("VCから退出できませんでした")?;
    ctx.send(|r| r.embed(|e| e.title("VCから退出しました").success_color()))
        .await?;

    Ok(())
}
