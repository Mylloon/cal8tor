use ics::{
    properties::{Class, Description, DtEnd, DtStart, Location, Summary, Transp},
    Event, ICalendar,
};

use chrono_tz::Europe::Paris;
use regex::Regex;

type T = (
    // Schedules
    Vec<String>,
    // Timetable per days with the semester as the key
    (usize, Vec<crate::timetable::models::Day>),
);
type D = std::collections::HashMap<
    usize,
    Vec<(chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)>,
>;

pub fn export(timetable: T, dates: D) -> ICalendar<'static> {
    let mut calendar = ICalendar::new("2.0", "cal8tor");

    // Create event which contains the information regarding the course
    let mut event = Event::new("uid", "dtstamp");

    // Add properties
    // Public event
    event.push(Class::public());
    // Consume actual time
    event.push(Transp::opaque());
    // Professor's name
    event.push(Description::new("Jean-Jacques Bourdin"));
    // Start time of the course
    event.push(DtStart::new("20220919T123000Z"));
    // End time of the course
    event.push(DtEnd::new("20220919T033000Z"));
    // Room location
    event.push(Location::new("A188"));
    // Course's name
    event.push(Summary::new("Algorithmique avancée"));

    // Add the course to the calendar
    calendar.add_event(event);

    calendar.save_file("target/debug2.ics").unwrap();

    let mut schedules = Vec::new();
    // h1 => heure de début | m1 => minute de début
    // h2 => heure de fin   | m2 => minute de fin
    let re =
        Regex::new(r"(?P<h1>\d{1,2})h(?P<m1>\d{1,2})?.(?P<h2>\d{1,2})h(?P<m2>\d{1,2})?").unwrap();
    for hour in timetable.0 {
        let captures = re.captures(&hour).unwrap();

        let h1 = match captures.name("h1") {
            Some(h) => h.as_str().parse().unwrap(),
            None => 0,
        };
        let m1 = match captures.name("m1") {
            Some(h) => h.as_str().parse().unwrap(),
            None => 0,
        };
        let h2 = match captures.name("h2") {
            Some(h) => h.as_str().parse().unwrap(),
            None => 0,
        };
        let m2 = match captures.name("m2") {
            Some(h) => h.as_str().parse().unwrap(),
            None => 0,
        };
        schedules.push(((h1, m1), (h2, m2)));
    }

    let semester = (timetable.1).0;
    let requested_timetable = (timetable.1).1;

    let mut requested_dates = Vec::new();
    for date in dates.get(&semester).unwrap() {
        requested_dates.push((date.0.with_timezone(&Paris), date.1.with_timezone(&Paris)));
    }

    println!("{:#?}", schedules);

    calendar
}
