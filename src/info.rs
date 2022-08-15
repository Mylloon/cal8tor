use scraper::Html;

pub async fn info() {
    let _document_info = get_webpage().await.expect("Can't reach info website.");
    // println!("{:#?}", document_info);
}

async fn get_webpage() -> Result<Html, Box<dyn std::error::Error>> {
    /* let html = reqwest::get("https://informatique.up8.edu/licence-iv/edt").await?.text().await?;

    Ok(Html::parse_document(&html)) */

    let html = include_str!("../target/debug2.html");
    Ok(Html::parse_document(html))
}
