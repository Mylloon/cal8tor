use ics::{
    properties::{Class, Description, DtEnd, DtStart, Location, Summary, Transp},
    Event, ICalendar,
};

pub fn export(courses: Vec<crate::timetable::models::Course>, filename: &str) {
    let mut calendar = ICalendar::new("2.0", "cal8tor");

    // Create events which contains the information regarding the course
    for course in courses {
        let mut event = Event::new("uid", "dtstamp");

        // Public event
        event.push(Class::public());
        // Consume actual time
        event.push(Transp::opaque());
        // Professor's name
        if course.professor.is_some() {
            event.push(Description::new(course.professor.unwrap()));
        }
        // Start time of the course
        event.push(DtStart::new(dt_ical(course.dtstart.unwrap())));
        // End time of the course
        event.push(DtEnd::new(dt_ical(course.dtend.unwrap())));
        // Room location
        event.push(Location::new(course.room));
        // Course's name
        event.push(Summary::new(course.name));

        // Add the course to the calendar
        calendar.add_event(event);
    }

    calendar.save_file(filename).unwrap();
}

/// Transform the datetime from chrono to the ICS format
/// See <https://github.com/hummingly/ics/issues/17#issue-985662287>
fn dt_ical(dt: chrono::DateTime<chrono::Utc>) -> String {
    format!("{}", dt.format("%Y%m%dT%H%M%SZ"))
}
