mod commands;
mod models;
mod schema;
mod db;

use commands::ping::PING_COMMAND;
use commands::help::HELP_COMMAND;
use commands::events;
use commands::on_join;

use db::setup::*;
use poise::serenity_prelude::Ready;
use serenity::prelude::*;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::guild::Member;
use serenity::framework::standard::StandardFramework;
use serenity::framework::standard::macros::group;

#[group]
#[commands(ping, help)]
struct General;

struct Bot {
    pool: PgPool
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        let _user_id = msg.author.id.0 as i64; // kept for now
        let cnn = &mut self.pool.get().expect("no connection allocated");

        if msg.content.trim() == ";events list" {
            events::list(&ctx, &msg, cnn).await;
        } else if let Some(event_id) = msg.content.strip_prefix(";events show") {
            let id = event_id.trim().parse::<i32>().unwrap();
            events::find(id, &ctx, &msg, cnn).await;
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

    async fn ready(&self, ctx: Context, _data: Ready) {
        ctx.set_activity(serenity::model::gateway::Activity::playing("Mario Kart")).await;
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

