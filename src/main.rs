use scraper::Html;

mod timetable;

#[tokio::main]
async fn main() {

    let _timetable = timetable::timetable(3, 1, None).await;

    let _document_info = get_webpage_info().await.expect("Can't reach info website.");
    // println!("{:#?}", document_info);
}

async fn get_webpage_info() -> Result<Html, Box<dyn std::error::Error>> {
    /* let html = reqwest::get("https://informatique.up8.edu/licence-iv/edt").await?.text().await?;

    Ok(Html::parse_document(&html)) */

    let html = include_str!("../target/debug2.html");
    Ok(Html::parse_document(html))
}
