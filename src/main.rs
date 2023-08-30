mod commands;
mod models;
mod schema;
mod db;

use commands::ping::PING_COMMAND;
use commands::help::HELP_COMMAND;

use commands::events;

use db::setup::*;
use serenity::prelude::*;
use serenity::async_trait;
use serenity::model::channel::Message;
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

        if msg.content.trim() == ";events list" {
            let cnn = &mut self.pool.get().expect("no connection allocated");
            events::list(&ctx, &msg, cnn).await;
        }
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
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

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

