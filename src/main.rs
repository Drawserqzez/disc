use std::fmt::Write as _;
mod models;
mod schema;
use diesel::pg::PgConnection;
use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager, PoolError };
use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};

#[group]
#[commands(ping)]
struct General;

struct Bot {
    pool: PgPool
}


pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        let user_id = msg.author.id.0 as i64;

        if msg.content.trim() == ";events list" {
            let cnn = &mut self.pool.get().expect("no connection allocated");

            let events = get_events(cnn).await;

            let mut response = format!("There are currently {} events\n", events.0.len());

            for (i, event) in events.0.iter().enumerate() {
                writeln!(response, "{}. {} @ {}", i + 1, event.name, event.event_time)
                    .unwrap();
            }

            msg.channel_id.say(&ctx, response).await.unwrap();
        }
    }
}

async fn get_events(cnn: &mut PgConnection) -> models::event::EventList {
    models::event::EventList::list(cnn)
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

fn establish_connection() -> PgPool {
    let database_url = dotenvy::var("DATABASE_URL").expect("No db url D:");

    init_pool(&database_url).expect("Failed to create pool ):")
}

fn init_pool(db_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder().build(manager)
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

