use std::fmt::Write as _;

use diesel::prelude::*;
use serenity::prelude::*;
use serenity::model::channel::Message;
use crate::{models::event::{Event, EventList, NewEvent}, schema::events};


pub async fn list(ctx: &Context, msg: &Message, cnn: &mut PgConnection) {
    let events = EventList::list(cnn);

    let mut response = format!("There are currently {} events\n", events.0.len());

    for (_, event) in events.0.iter().enumerate() {
        writeln!(response, "{}. {} @ {}", event.id, event.name, event.event_time)
            .unwrap();
    }

    msg.reply(&ctx, response).await.unwrap();
}

pub async fn find(id: i32, ctx: &Context, msg: &Message, cnn: &mut PgConnection) {
    let event = Event::find(id, cnn);

    let response = match event {
        Some(e) => fancyformat_event(e),
        None => format!("No event with id {} was found", id)
    };

    msg.reply(&ctx, response).await.unwrap();
}

pub async fn create(event: NewEvent, ctx: &Context, msg: &Message, cnn: &mut PgConnection) {
    let created_event = diesel::insert_into(events::table)
        .values(&event)
        .returning(Event::as_returning())
        .get_result(cnn);

    let response = match created_event {
        Ok(e) => fancyformat_event(e),
        Err(err) => format!("Error creating event: {}", err) 
    };

    msg.reply(&ctx, response).await.unwrap();
}

fn fancyformat_event(event: Event) -> String {
    format!(
        "```json\n{{\n  id: {}\n  name: {}\n  time: {}\n}}\n```", 
        event.id, 
        event.name, 
        event.event_time
    )
}

