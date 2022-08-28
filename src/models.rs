use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema::pd_events;
use crate::schema::pd_events::dsl::pd_events as all_pd_events;

#[derive(Serialize, Queryable, Debug, Clone)]
pub struct Event {
    pub id: i32,
    pub description: String,
    pub additional_info: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "pd_events"]
pub struct NewEvent {
    pub description: String,
    pub additional_info: String,
}

impl Event {
    pub fn show(id: i32, conn: &PgConnection) -> Vec<Event> {
        all_pd_events
            .find(id)
            .load::<Event>(conn)
            .expect("Error loading event")
    }

    pub fn all(conn: &PgConnection) -> Vec<Event> {
        all_pd_events
            .order(pd_events::id.desc())
            .load::<Event>(conn)
            .expect("Error loading event")
    }

    pub fn update_by_id(id: i32, conn: &PgConnection, new_event: NewEvent) -> bool {
        use crate::schema::pd_events::dsl::{additional_info as a, description as d};
        let NewEvent {
            description,
            additional_info,
        } = new_event;

        diesel::update(all_pd_events.find(id))
            .set((a.eq(description), d.eq(additional_info)))
            .get_result::<Event>(conn)
            .is_ok()
    }

    pub fn insert(new_event: NewEvent, conn: &PgConnection) -> bool {
        diesel::insert_into(pd_events::table)
            .values(&new_event)
            .execute(conn)
            .is_ok()
    }

    pub fn delete_by_id(id: i32, conn: &PgConnection) -> bool {
        if Event::show(id, conn).is_empty() {
            return false;
        };
        diesel::delete(all_pd_events.find(id)).execute(conn).is_ok()
    }
}
