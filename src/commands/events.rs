use std::fmt::Write as _;

use diesel::PgConnection;
use serenity::prelude::*;
use serenity::model::channel::Message;
use crate::models;

pub async fn list(ctx: &Context, msg: &Message, cnn: &mut PgConnection) {
    let events = models::event::EventList::list(cnn);

    let mut response = format!("There are currently {} events\n", events.0.len());

    for (_, event) in events.0.iter().enumerate() {
        writeln!(response, "{}. {} @ {}", event.id, event.name, event.event_time)
            .unwrap();
    }

    msg.channel_id.say(&ctx, response).await.unwrap();
}

