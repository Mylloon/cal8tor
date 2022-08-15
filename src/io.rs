use ics::{Event, ICalendar};

type T = Vec<crate::timetable::models::Day>;
type D = std::collections::HashMap<
    usize,
    Vec<(chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)>,
>;

pub fn export(_timetable: T, _dates: D) -> ICalendar<'static> {
    let mut calendar = ICalendar::new("2.0", "cal8tor");

    let event = Event::new("uid", "dtstamp");

    calendar.add_event(event);

    calendar.save_file("target/debug2.ics").unwrap();

    calendar
}
