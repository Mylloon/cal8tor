#[derive(Debug)]
pub struct Course {
    /// Professor's name
    pub professor: String,

    /// List of rooms where the course takes place
    pub room: Vec<String>,

    /// Time the course starts, as a number :
    /// - 0 => first possible class of the day
    /// - 1 => second possible class of the day
    /// - etc.
    pub start: i8,

    /// Number of time slots the course takes up in the timetable
    pub size: i8,
}
#[derive(Debug)]
pub struct Day {
    /// Day's name
    pub name: String,
    /// Ordered list of all the courses of the day
    pub courses: Vec<Option<Course>>,
}
