pub type CommandResult = anyhow::Result<()>;
pub type Context<'a> = poise::Context<'a, crate::shared::Data, anyhow::Error>;

mod connect;
pub use connect::connect;

mod disconnect;
pub use disconnect::disconnect;
