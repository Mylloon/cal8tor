use scraper::{Html, Selector};

pub async fn info() {
    let document = get_webpage().await.expect("Can't reach info website.");

    // Selectors
    let sel_ul = Selector::parse("ul").unwrap();

    // Find the raw infos in html page
    for (i, data) in document.select(&sel_ul).enumerate() {
        if [1, 2].contains(&i) {
            println!("\n{} - {:#?}", data.value().name(), data.inner_html());
        }
    }
}

async fn get_webpage() -> Result<Html, Box<dyn std::error::Error>> {
    /* let html = reqwest::get("https://informatique.up8.edu/licence-iv/edt").await?.text().await?;

    Ok(Html::parse_document(&html)) */

    let html = include_str!("../target/debug2.html");
    Ok(Html::parse_document(html))
}
