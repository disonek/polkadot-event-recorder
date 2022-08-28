use super::models::{Event, NewEvent};
use crate::database::Conn as DbConn;
use rocket_contrib::json::Json;
use serde_json::Value;

#[get("/pd_events", format = "json")]
pub fn index(conn: DbConn) -> Json<Value> {
    let pd_events = Event::all(&conn);

    Json(json!({
        "status": 200,
        "result": pd_events,
    }))
}

#[post("/pd_events", format = "json", data = "<new_event>")]
pub fn new(conn: DbConn, new_event: Json<NewEvent>) -> Json<Value> {
    Json(json!({
        "status": Event::insert(new_event.into_inner(), &conn),
        "result": Event::all(&conn).first(),
    }))
}

#[get("/pd_events/<id>", format = "json")]
pub fn show(conn: DbConn, id: i32) -> Json<Value> {
    let result = Event::show(id, &conn);
    let status = if result.is_empty() { 404 } else { 200 };

    Json(json!({
        "status": status,
        "result": result.get(0),
    }))
}

#[put("/pd_events/<id>", format = "json", data = "<event>")]
pub fn update(conn: DbConn, id: i32, event: Json<NewEvent>) -> Json<Value> {
    let status = if Event::update_by_id(id, &conn, event.into_inner()) {
        200
    } else {
        404
    };

    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[delete("/pd_events/<id>")]
pub fn delete(id: i32, conn: DbConn) -> Json<Value> {
    let status = if Event::delete_by_id(id, &conn) {
        200
    } else {
        404
    };
    Json(json!({
        "status": status,
        "result": null,
    }))
}

//#[catch(404)]
//fn not_found() -> Json<Value> {
//    Json(json!({
//        "status": "error",
//        "reason": "Resource was not found"
//    }))
//}
//needs to be reworked for current version of rocket.
