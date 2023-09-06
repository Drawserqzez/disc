use poise::serenity_prelude::CommandDataOption;
use serenity::builder::CreateApplicationCommand;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;

#[command]
pub async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Sike you thought").await?;

    Ok(())
}


pub fn run(_options: &[CommandDataOption]) -> String {
    "Sike you thought!".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("help").description("Help")
}
