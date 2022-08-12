use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let url = "https://informatique.up8.edu/licence-iv/edt/l3.html";

    // Get raw html
    //let html = reqwest::get(url).await?.text().await?;
    let html = include_str!("../target/debug.html");

    // Parse document
    let document = Html::parse_document(&html);

    // Find the timetable
    let selector = Selector::parse("table").unwrap();
    let raw_timetable = document.select(&selector);

    println!("{:#?}", raw_timetable);

    Ok(())
}
