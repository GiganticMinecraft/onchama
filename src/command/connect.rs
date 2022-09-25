use crate::{
    command::{CommandResult, Context},
    shared::CreateDiscordEmbedExt,
};

use anyhow::Context as _;
use poise::serenity_prelude::ChannelId;

/// 読み上げを開始します
#[poise::command(slash_command)]
pub async fn connect(ctx: Context<'_>) -> CommandResult {
    let guild = ctx.guild().context("Guildがありません")?;
    let guild_id = guild.id;
    // TODO: fix comment out
    // let channel_id = guild
    //     .voice_states.get(&msg.author.id)
    //     .and_then(|voice_state| voice_state.channel_id)
    //     .context("VCに参加してください")?;
    let vc_id = ChannelId::from(872720546742296667);
    let manager = songbird::get(&ctx.discord())
        .await
        .context("Songbirdが初期化されていません")?;
    let _ = manager.join(guild_id, vc_id).await;
    ctx.data().text_ch.save(ctx.channel_id());
    ctx.send(|b| {
        b.embed(|e| {
            let vc = guild.channels.get(&vc_id).unwrap().clone();
            e.success_color()
                .title(format!("{}に接続しました", vc.guild().unwrap().name))
        })
    })
    .await?;

    Ok(())
}
