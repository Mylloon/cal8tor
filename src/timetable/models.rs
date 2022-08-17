use tabled::Tabled;

#[derive(Tabled, Clone, Debug)]
#[tabled(rename_all = "CamelCase")]
pub struct Course {
    /// Course's name
    pub name: String,

    /// Professor's name
    #[tabled(display_with = "crate::utils::display_option")]
    pub professor: Option<String>,

    /// List of rooms where the course takes place
    pub room: String,

    /// Time the course starts, as a number :
    /// - 0 => first possible class of the day
    /// - 1 => second possible class of the day
    /// - etc.
    #[tabled(skip)]
    pub start: usize,

    /// Number of time slots the course takes up in the timetable
    #[tabled(skip)]
    pub size: usize,

    /// Datetime when the course start
    /// Filled only when building for the ICS
    #[tabled(skip)]
    pub dtstart: Option<chrono::DateTime<chrono::Utc>>,

    /// Datetime when the course end
    /// Filled only when building for the ICS
    #[tabled(skip)]
    pub dtend: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Tabled, Debug)]
pub struct Day {
    /// Day's name
    pub name: String,
    /// Ordered list of all the courses of the day
    #[tabled(skip)]
    pub courses: Vec<Option<Course>>,
}
