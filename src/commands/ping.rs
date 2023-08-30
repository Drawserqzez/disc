use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

