use ics::{
    properties::{Class, Description, DtEnd, DtStart, Location, Summary, Transp},
    Event, ICalendar,
};

pub fn export(timetable: Vec<String>, filename: &str) {
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

    calendar.save_file(filename).unwrap();
}
