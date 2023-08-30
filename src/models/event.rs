use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::{Queryable, PgConnection, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Event {
    pub id: i32,
    pub name: String, 
    pub event_time: DateTime<Utc>,
    pub created_time: Option<DateTime<Utc>>
}

pub struct EventList(pub Vec<Event>);

impl EventList {
    pub fn list(cnn: &mut PgConnection) -> Self {
        use crate::schema::events::dsl::*;

        let result = 
            events
            .select(Event::as_select())
            .load(cnn)
            .expect("gamer");

        EventList(result)
    }
}
