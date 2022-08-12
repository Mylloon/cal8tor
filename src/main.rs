use scraper::{Html, Selector};

mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let url = "https://informatique.up8.edu/licence-iv/edt/l3.html";

    // Get raw html
    //let html = reqwest::get(url).await?.text().await?;
    let html = include_str!("../target/debug.html");

    // Parse document
    let document = Html::parse_document(&html);

    // Find the timetable
    let selector_timetable = Selector::parse("table").unwrap();
    let raw_timetable = document.select(&selector_timetable).next().unwrap();

    //println!("{}", &raw_timetable.inner_html());

    // Find the slots available for the timetable
    let selector_schedules = Selector::parse("tr").unwrap();
    let raw_schedules = raw_timetable.select(&selector_schedules).next().unwrap();
    println!("{}", &raw_schedules.inner_html());


    Ok(())
}
