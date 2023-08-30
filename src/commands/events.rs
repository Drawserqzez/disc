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

    msg.reply(&ctx, response).await.unwrap();
}

pub async fn find(id: i32, ctx: &Context, msg: &Message, cnn: &mut PgConnection) {
    let event = models::event::Event::find(id, cnn);

    let response = match event {
        Some(e) => fancyformat_event(e),
        None => format!("No event with id {} was found", id)
    };

    msg.reply(&ctx, response).await.unwrap();
}

fn fancyformat_event(event: models::event::Event) -> String {
    format!(
        "```json\n{{\n  id: {}\n  name: {}\n  time: {}\n}}\n```", 
        event.id, 
        event.name, 
        event.event_time
    )
}

