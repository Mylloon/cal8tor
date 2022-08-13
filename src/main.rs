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
    let sel_tr    = Selector::parse("tr").unwrap();
    let sel_tbody = Selector::parse("tbody").unwrap();
    let sel_th    = Selector::parse("th").unwrap();
    let sel_td    = Selector::parse("td").unwrap();

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
        for course in day.select(&sel_td) {
            if course.inner_html() == "—" {
                courses_vec.push(None);
            } else {
                courses_vec.push(Some(models::Course {
                    professor: "coucou".to_string(),
                    room: Vec::new(),
                    start: 0,
                    size: 1,
                }));
            }
        }

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
    // No work during week-end
    if timetable.len() == 5 {
        // TODO: Check if schedules.len() is coherent
        // with the values inside the timetable
        println!("{:#?}", schedules);
        println!("{:#?}", timetable);
        return true;
    }

    false
}
