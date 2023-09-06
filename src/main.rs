mod commands;
mod models;
mod schema;
mod db;

use chrono::{DateTime, Utc};
use commands::bing::BING_COMMAND;
use commands::ping::PING_COMMAND;
use commands::help::HELP_COMMAND;
use commands::events;
use commands::on_join;

use db::setup::*;
use models::event::NewEvent;
use poise::serenity_prelude::GuildId;
use poise::serenity_prelude::Ready;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::prelude::*;
use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::channel::Message;
use serenity::model::guild::Member;
use serenity::framework::standard::StandardFramework;
use serenity::framework::standard::macros::group;

#[group]
#[commands(bing, ping, help)]
struct General;

struct Bot {
    pool: PgPool
}

#[async_trait]
impl EventHandler for Bot {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "help" => commands::help::run(&command.data.options),
                _ => "not implemented LOL".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx, |resp| {
                    resp.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|msg| msg.content(content))
                }) 
            .await
            {
                print!("error responding to slash command: {}", why);
            }
        }
    }
    async fn message(&self, ctx: Context, msg: Message) {
        let user_id = msg.author.id.0 as i64; // kept for now
        let cnn = &mut self.pool.get().expect("no connection allocated");

        if msg.content.trim() == ";events list" {
            events::list(&ctx, &msg, cnn).await;
        } else if let Some(event_id) = msg.content.strip_prefix(";events show") {
            let id = event_id.trim().parse::<i32>().unwrap();
            events::find(id, &ctx, &msg, cnn).await;
        } else if let Some(event_data) = msg.content.strip_prefix(";events add") {
            let creator = user_id;
            let data: Vec<&str> = event_data.split_whitespace()
                .collect();

            // todo: fix the date time format, probably split date and time 
            let time = DateTime::parse_from_str(data[1], "%Y-%m-%d@%H:%M");

            if let Err(_) = time {
                msg.reply(&ctx, format!("Felaktigt datumformat: {}", data[1])).await.unwrap();
                return;
            }

            let utc_time = time.unwrap().with_timezone::<Utc>(&Utc);

            let new_event = NewEvent {
                name: data[0].to_string(),
                owner: creator,
                event_time: utc_time,
                domain: data[2].to_string()
            };

            events::create(new_event, &ctx, &msg, cnn).await;
        } else if msg.content.trim() == ";welcometest" {
            let mut member = msg.guild(&ctx).unwrap()
                .member(&ctx, msg.author.id)
                .await.unwrap();

            on_join::choose_activities(&ctx, &mut member, &msg.guild_id.unwrap())
                .await;
        }
    }

    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        let mut member = new_member.clone();
        on_join::choose_activities(&ctx, &mut member, &new_member.guild_id)
            .await;
    }

    async fn ready(&self, ctx: Context, data: Ready) {
        ctx.set_activity(serenity::model::gateway::Activity::playing("Mario Kart")).await;

        for guild in data.guilds {
            let _ = GuildId::set_application_commands(&guild.id, &ctx, |cmds| {
                cmds.create_application_command(|cmd| commands::help::register(cmd))
            })
            .await;
        }

        let _ = Command::create_global_application_command(&ctx, |cmd| {
            commands::help::register(cmd)
        })
        .await;
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let pg_cnn = establish_connection(); 

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(";"))
        .group(&GENERAL_GROUP);

    let token = dotenvy::var("DISCORD_TOKEN").expect("Need a token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILDS | GatewayIntents::GUILD_MEMBERS;

    let bot = Bot {
        pool: pg_cnn
    };

    let mut client = Client::builder(token, intents)
        .event_handler(bot)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

