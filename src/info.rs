use regex::Regex;
use scraper::{Html, Selector};

pub async fn info() {
    let document = get_webpage().await.expect("Can't reach info website.");

    // Selectors
    let sel_ul = Selector::parse("ul").unwrap();
    let sel_li = Selector::parse("li").unwrap();

    // Find the raw infos in html page
    let mut raw_data = Vec::new();
    for (i, data) in document.select(&sel_ul).enumerate() {
        if [1, 2].contains(&i) {
            raw_data.push(data);
        }
    }

    let mut data = std::collections::HashMap::new();
    // d => date
    // r => repetition
    let re = Regex::new(r"(?P<d>\d{1,2} \w+ \d{4}).+(?P<r>\d)").unwrap();
    for (i, ul) in raw_data.into_iter().enumerate() {
        for element in ul.select(&sel_li) {
            match element.inner_html() {
                e if e.starts_with("Début") => {
                    let captures = re.captures(&e).unwrap();
                    data.insert(
                        i,
                        format!(
                            "{} pendant {}s",
                            captures.name("d").unwrap().as_str(),
                            captures.name("r").unwrap().as_str()
                        ),
                    );
                }
                e if e.starts_with("Reprise") => {
                    let captures = re.captures(&e).unwrap();
                    captures.name("g");
                    data.insert(
                        i,
                        format!(
                            "{} puis reprise {} pendant {}s",
                            data.get(&i).unwrap(),
                            captures.name("d").unwrap().as_str(),
                            captures.name("r").unwrap().as_str()
                        ),
                    );
                }
                _ => (),
            }
        }
    }

    println!("{:#?}", data);
}

async fn get_webpage() -> Result<Html, Box<dyn std::error::Error>> {
    /* let html = reqwest::get("https://informatique.up8.edu/licence-iv/edt").await?.text().await?;

    Ok(Html::parse_document(&html)) */

    let html = include_str!("../target/debug2.html");
    Ok(Html::parse_document(html))
}
