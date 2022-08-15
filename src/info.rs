use std::collections::HashMap;

use chrono::{TimeZone, Utc};
use regex::{Captures, Regex};
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

    let mut data = HashMap::new();
    // d => date
    // r => repetition
    let re = Regex::new(r"(?P<d>\d{1,2} \w+ \d{4}).+(?P<r>\d)").unwrap();
    for (i, ul) in raw_data.into_iter().enumerate() {
        for element in ul.select(&sel_li) {
            match element.inner_html() {
                e if e.starts_with("Début") => {
                    let captures = re.captures(&e).unwrap();

                    let date = captures.name("d").unwrap().as_str();
                    let rep = captures.name("r").unwrap().as_str();

                    let from = Utc
                        .datetime_from_str(&anglophonization(date), "%e %B %Y %H:%M")
                        .unwrap();

                    println!("'{}' -> {}", date, from);

                    data.insert(i, format!("{} pendant {}s", date, rep));
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

/// Get info webpage
async fn get_webpage() -> Result<Html, Box<dyn std::error::Error>> {
    /* let html = reqwest::get("https://informatique.up8.edu/licence-iv/edt").await?.text().await?;

    Ok(Html::parse_document(&html)) */

    let html = include_str!("../target/debug2.html");
    Ok(Html::parse_document(html))
}

/// Turn a french date to an english one
fn anglophonization(date: &str) -> String {
    let dico = HashMap::from([
        ("janvier", "january"),
        ("mars", "march"),
        ("septembre", "september"),
        ("novembre", "november"),
    ]);

    // New regex of all the french month
    let re = Regex::new(&format!(
        "({})",
        dico.keys().cloned().collect::<Vec<_>>().join("|")
    ))
    .unwrap();

    format!(
        // Use 12:00 for chrono parser
        "{} 12:00",
        // Replace french by english month
        re.replace_all(date, |cap: &Captures| {
            match &cap[0] {
                month if dico.contains_key(month) => dico.get(month).unwrap(),
                month => panic!("Unknown month: {}", month),
            }
        })
    )
}
