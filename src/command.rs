pub type CommandResult = anyhow::Result<()>;
pub type Context<'a> = poise::Context<'a, crate::Data, anyhow::Error>;

mod connect;
pub use connect::connect;
