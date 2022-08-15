use std::collections::HashMap;

use chrono::{Duration, TimeZone, Utc};
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

                    let start_date = get_date(captures.name("d").unwrap().as_str());

                    let rep: i64 = captures.name("r").unwrap().as_str().parse().unwrap();
                    // -1 car la première semaine est déjà compté
                    let end_date = start_date + Duration::weeks(rep - 1);

                    data.insert(i, vec![(start_date, end_date)]);
                }
                e if e.starts_with("Reprise") => {
                    let captures = re.captures(&e).unwrap();
                    captures.name("g");

                    let start_date = get_date(captures.name("d").unwrap().as_str());

                    let rep: i64 = captures.name("r").unwrap().as_str().parse().unwrap();
                    // -1 car la première semaine est déjà compté
                    let end_date = start_date + Duration::weeks(rep - 1);

                    let mut vec = data.get(&i).unwrap().to_owned();
                    vec.push((start_date, end_date));

                    data.insert(i, vec);
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

/// Turn a string to a DateTime
fn get_date(date: &str) -> chrono::DateTime<Utc> {
    // Use and keep UTC time, we have the hour set to 12h and
    // Paris 8 is in France so there is no problems
    Utc.datetime_from_str(&anglophonization(date), "%e %B %Y %H:%M")
        .unwrap()
}
