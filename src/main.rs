use scraper::{Html, Selector};

mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let url = "https://informatique.up8.edu/licence-iv/edt/l3.html";

    // Get raw html
    //let html = reqwest::get(url).await?.text().await?;
    let html = include_str!("../target/debug.html");

    // Parse document
    let document = Html::parse_document(html);

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
                    room: course.select(&sel_strong).next().unwrap().inner_html(),
                    start: location_tracker,
                    size: match course.value().attr("colspan") {
                        Some(i) => i.parse().unwrap(),
                        None => 1,
                    },
                }));

                match &courses_vec[courses_vec.len() - 1] {
                    Some(course) => location_tracker += course.size,
                    None => location_tracker += 1,
                }
            }
        }
        println!("\n");

        timetable.push(models::Day {
            name: day.select(&sel_th).next().unwrap().inner_html(),
            courses: courses_vec,
        })
    }

    // TODO: Make fn who chacke if timetable is bell built (time consistency)
    if !check_timetable_consistency(&schedules, &timetable) {
        panic!("Error when building the timetable.");
    }

    Ok(())
}

/// Check if the timetable is well built
fn check_timetable_consistency(schedules: &Vec<String>, timetable: &Vec<models::Day>) -> bool {
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
