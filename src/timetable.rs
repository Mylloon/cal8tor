use chrono::{TimeZone, Utc};
use regex::Regex;
use scraper::{Html, Selector};

pub mod models;

/// Fetch the timetable for a class
pub async fn timetable(
    year: i8,
    semester: i8,
    letter: Option<char>,
) -> (Vec<String>, (usize, Vec<models::Day>)) {
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
                    name: course.select(&sel_em).next().unwrap().inner_html(),
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
                let c = letter.expect(panic_letter_message);
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
                let c = letter.expect(panic_letter_message);
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
    let html = reqwest::get(url).await?.text().await?;

    // Parse document
    let document = Html::parse_document(&html); */

    println!("Fetch 'L{}{} (s{})'", year, letter.unwrap_or(' '), semester);
    let html = include_str!("../target/debug.html");
    let document = Html::parse_document(html);

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
type D = std::collections::HashMap<
    // Semester
    usize,
    // List of start and end times of course weeks
    Vec<(chrono::DateTime<Utc>, chrono::DateTime<Utc>)>,
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

    let semester = (timetable.1).0;
    let requested_timetable = (timetable.1).1;

    let _requested_dates = dates.get(&semester).unwrap();

    println!("{:#?}", requested_timetable);

    vec![models::Course {
        name: String::from("Cours"),
        professor: None,
        room: String::from("A188"),
        start: 0,
        size: 0,
        dtstart: Some(Utc.ymd(2022, 9, 19).and_hms(13, 30, 0)),
        dtend: Some(Utc.ymd(2022, 9, 19).and_hms(16, 30, 0)),
    }]
}
