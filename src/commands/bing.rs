use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;

#[command]
pub async fn bing(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "#速度与激情9#\n\n早上好中国\n现在我有冰激淋 我很喜欢冰激淋\n但是《速度与激情9》比冰激淋……\n🍦").await?;

    Ok(())
}

