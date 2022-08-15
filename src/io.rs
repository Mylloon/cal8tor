use ics::ICalendar;

pub fn export(
    timetable: Vec<crate::timetable::models::Day>,
    dates: std::collections::HashMap<
        usize,
        Vec<(chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)>,
    >,
) -> ICalendar<'static> {
    let mut calendar = ICalendar::new("2.0", "cal8tor");

    calendar
}
