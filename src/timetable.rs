use chrono::{Datelike, Duration, TimeZone, Utc};
use regex::Regex;
use scraper::{Html, Selector};
use std::collections::HashMap;

use crate::utils::{
    self,
    models::{Position, TabChar},
};

pub mod models;

/// Fetch the timetable for a class
pub async fn timetable(
    year: i8,
    semester_opt: Option<i8>,
    letter: Option<char>,
) -> (Vec<String>, (usize, Vec<models::Day>)) {
    let semester = get_semester(semester_opt, letter);

    let document = get_webpage(year, semester, letter)
        .await
        .expect("Can't reach timetable website.");

    // Selectors
    let sel_table = Selector::parse("table").unwrap();
    let sel_tr = Selector::parse("tr").unwrap();
    let sel_tbody = Selector::parse("tbody").unwrap();
    let sel_th = Selector::parse("th").unwrap();
    let sel_td = Selector::parse("td").unwrap();
    let sel_em = Selector::parse("em").unwrap();
    let sel_small = Selector::parse("small").unwrap();
    let sel_strong = Selector::parse("strong").unwrap();

    // Find the timetable
    let raw_timetable = document.select(&sel_table).next().unwrap();

    // Find the slots available for the timetable
    let raw_schedules = raw_timetable.select(&sel_tr).next().unwrap();

    // Find availables schedules
    let mut schedules = Vec::new();
    for time in raw_schedules.select(&sel_th) {
        schedules.push(time.inner_html());
    }

    // Find the timetable values
    let raw_timetable_values = raw_timetable.select(&sel_tbody).next().unwrap();

    // For each days
    let mut timetable = Vec::new();
    for day in raw_timetable_values.select(&sel_tr) {
        let mut courses_vec = Vec::new();
        let mut location_tracker = 0;
        for course in day.select(&sel_td) {
            if course.inner_html() == "—" {
                courses_vec.push(None);
                location_tracker += 1;
            } else {
                courses_vec.push(Some(models::Course {
                    name: match course.select(&sel_em).next() {
                        Some(value) => value.inner_html(),
                        None => course
                            .inner_html()
                            .split("<br>")
                            .next()
                            .unwrap()
                            .to_string(),
                    },
                    professor: match course
                        .select(&sel_small)
                        .next()
                        .unwrap()
                        .inner_html()
                        .split("<br>")
                        .next()
                    {
                        Some(data) => {
                            if data.contains("</strong>") {
                                // This is the room, so there is no professor assigned
                                // to this courses yet
                                None
                            } else {
                                Some(data.to_string())
                            }
                        }
                        None => None,
                    },
                    room: course
                        .select(&sel_strong)
                        .next()
                        .unwrap()
                        .inner_html()
                        .replace("<br>", ""),
                    start: location_tracker,
                    size: match course.value().attr("colspan") {
                        Some(i) => i.parse().unwrap(),
                        None => 1,
                    },
                    dtstart: None,
                    dtend: None,
                }));

                match &courses_vec[courses_vec.len() - 1] {
                    Some(course) => location_tracker += course.size,
                    None => location_tracker += 1,
                }
            }
        }

        timetable.push(models::Day {
            name: day.select(&sel_th).next().unwrap().inner_html(),
            courses: courses_vec,
        })
    }

    if !check_consistency(&schedules, &timetable) {
        panic!("Error when building the timetable.");
    }

    (schedules, (semester as usize, timetable))
}

/// Get timetable webpage
async fn get_webpage(
    year: i8,
    semester: i8,
    letter: Option<char>,
) -> Result<Html, Box<dyn std::error::Error>> {
    /* let url = {
        let panic_semester_message = "Unknown semester.";
        let panic_letter_message = "Unknown letter.";

        let base_url = "https://informatique.up8.edu/licence-iv/edt";
        match year {
            1 => {
                let allow_letters = match semester {
                    1 => ['a', 'b', 'c'],
                    2 => ['x', 'y', 'z'],
                    _ => panic!("{}", panic_semester_message),
                };
                let c = letter.expect(panic_letter_message).to_ascii_lowercase();
                if allow_letters.contains(&c) {
                    format!("{}/l1-{}.html", base_url, c)
                } else {
                    panic!("{}", panic_letter_message)
                }
            }
            2 => {
                let allow_letters = match semester {
                    1 => ['a', 'b'],
                    2 => ['x', 'y'],
                    _ => panic!("{}", panic_semester_message),
                };
                let c = letter.expect(panic_letter_message).to_ascii_lowercase();
                if allow_letters.contains(&c) {
                    format!("{}/l2-{}.html", base_url, c)
                } else {
                    panic!("{}", panic_letter_message)
                }
            }
            3 => match semester {
                1 => format!("{}/l3.html", base_url),
                2 => format!("{}/l3_2.html", base_url),
                _ => panic!("{}", panic_semester_message),
            },
            _ => panic!("Unknown year."),
        }
    };

    // Get raw html
    let html = reqwest::get(&url).await?.text().await?;

    // Panic on error
    crate::utils::check_errors(&html, &url); */
    let html = std::fs::read_to_string("target/debug-l1a.htm").unwrap();

    // Parse document
    let document = Html::parse_document(&html);

    Ok(document)
}

/// Check if the timetable is well built
fn check_consistency(schedules: &Vec<String>, timetable: &Vec<models::Day>) -> bool {
    let mut checker = true;
    for day in timetable {
        let mut i = 0;
        for course in &day.courses {
            match course {
                Some(course_it) => {
                    // Checks the consistency of course start times
                    if i != course_it.start {
                        checker = false;
                        break;
                    }
                    // Keep the track of how many courses are in the day
                    i += course_it.size
                }
                None => i += 1,
            }
        }
        // The counter should be the same as the amount of possible hours of the day
        if i != schedules.len() {
            checker = false;
            break;
        }
    }

    checker
}

// Data builded in the timetable webpage
type T = (
    // Schedules
    Vec<String>,
    // Timetable per days with the semester as the key
    (usize, Vec<models::Day>),
);
// Data builded in the info webpage
type D = HashMap<
    // Semester
    usize,
    // List of start and repetition of course weeks
    Vec<(chrono::DateTime<Utc>, i64)>,
>;

/// Build the timetable
pub fn build(timetable: T, dates: D) -> Vec<models::Course> {
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

    // Store all the courses for the semester
    let mut semester = Vec::new();

    // Start date of the back-to-school week
    let datetimes = dates.get(&timetable.1 .0).unwrap();
    let before_break = datetimes.get(0).unwrap();
    let mut date = before_break.0;
    let mut rep = before_break.1;
    // For each weeks
    for _ in 0..2 {
        for _ in 0..rep {
            for day in &timetable.1 .1 {
                for mut course in day.courses.clone().into_iter().flatten() {
                    // Get the hours
                    let start = schedules.get(course.start).unwrap().0;
                    // -1 because we only add when the size is > 1
                    let end = schedules.get(course.start + course.size - 1).unwrap().1;

                    // Add the changed datetimes
                    course.dtstart = Some(
                        Utc.ymd(date.year(), date.month(), date.day())
                            .and_hms(start.0, start.1, 0),
                    );
                    course.dtend = Some(
                        Utc.ymd(date.year(), date.month(), date.day())
                            .and_hms(end.0, end.1, 0),
                    );

                    semester.push(course);
                }
                date += Duration::days(1);
            }
            // From friday to monday
            date += Duration::days(2);
        }
        let after_break = datetimes.get(1).unwrap();
        date = after_break.0;
        rep = after_break.1;
    }

    semester
}

/// Get the current semester depending on the letter or the current date
fn get_semester(semester: Option<i8>, letter: Option<char>) -> i8 {
    match semester {
        // Force the asked semester
        Some(n) => n,
        // Find the potential semester
        None => match letter {
            // Based on letter (kinda accurate)
            Some(c) => {
                if c as i8 > 77 {
                    // If letter is N or after
                    2
                } else {
                    // If letter is before N
                    1
                }
            }
            // Based on the time (kinda less accurate)
            None => {
                if Utc::now().month() > 6 {
                    // From july to december
                    1
                } else {
                    // from january to june
                    2
                }
            }
        },
    }
}

/// Display the timetable
pub fn display(timetable: (Vec<String>, (usize, Vec<models::Day>))) {
    // Cell length
    let cl = 35;
    // Cell length for hours
    let clh = 11;
    // Cell number
    let cn = 6;

    let sep = TabChar::Bv.val();

    // Top of the tab
    utils::line_table(clh, cl, cn, Position::Top, HashMap::new());

    // First empty case
    print!("{}{:^clh$}{}", sep, "", sep);

    // Print day's of the week
    let mut days = HashMap::new();
    for (i, data) in (&timetable.1 .1).iter().enumerate() {
        days.insert(i, &data.name);
        print!("{:^cl$}{}", &data.name, sep);
    }

    // Store the data of the course for utils::line_table
    let mut next_skip = HashMap::new();
    // For each hours
    for (i, hour) in timetable.0.into_iter().enumerate() {
        // Draw separator line
        utils::line_table(clh, cl, cn, Position::Middle, next_skip);

        // Reset
        next_skip = HashMap::new();

        // Print hour
        print!("{}{:^clh$}", sep, hour);

        // For all the days
        for (j, day) in (&timetable.1 .1).iter().enumerate() {
            // For all the courses of each days
            for (k, course_opt) in (&day.courses).iter().enumerate() {
                match course_opt {
                    // If there is a course
                    Some(course) => {
                        // Check if the course's hour
                        if i == k {
                            if course.size != 1 {
                                // If the course uses more than one time slot
                                next_skip.insert(course.start, &course.name);
                                print!("{}{:^cl$}", sep, "");
                                break;
                            } else {
                                // Else simply print the course
                                print!("{}{:^cl$}", sep, &course.name);
                                break;
                            }
                        }
                    }
                    // If no course was found
                    None => {
                        // Verify the "no course" is in the correct day and hour
                        if *days.get(&j).unwrap() == &day.name.to_string() && k == i {
                            // If yes print empty row
                            print!("{}{:^cl$}", sep, "");
                            break;
                        }
                        // Else it was a course of another day/time
                    }
                };
            }
        }
        print!("{}", sep);
    }
    // Bottom of the table
    utils::line_table(clh, cl, cn, Position::Bottom, HashMap::new());
}
