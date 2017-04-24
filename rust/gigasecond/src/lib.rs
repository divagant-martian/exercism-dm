extern crate chrono;
use chrono::*;

pub fn after(start_date:chrono::DateTime<chrono::UTC>) -> chrono::DateTime<chrono::UTC> {
    start_date + chrono::Duration::seconds(1000000000)

}
