use clap::Parser;
use regex::Regex;

mod ics;
mod info;
mod timetable;
mod utils;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Args {
    /// The class you want to get the timetable, i.e.: L2-A
    #[clap(value_parser)]
    class: String,

    /// The semester you want (useful only in 3rd year, 1-2 use letter in class)
    #[clap(short, long, value_parser, value_name = "SEMESTER NUMBER")]
    semester: Option<i8>,

    /// Export to iCalendar format (.ics)
    #[clap(short, long, value_name = "FILE NAME")]
    export: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let matches = Regex::new(r"[Ll](?P<year>\d)[-–•·]?(?P<letter>.)?")
        .unwrap()
        .captures(&args.class)
        .unwrap();

    let year = matches
        .name("year")
        .unwrap()
        .as_str()
        .parse::<i8>()
        .unwrap();
    let letter = matches
        .name("letter")
        .map(|c| c.as_str().chars().next().expect("Error in letter"));

    // Show a separator only if we need one
    let seperator = match letter {
        Some(_) => "-",
        None => "",
    };

    println!(
        "Récupération de l'emploi du temps des L{}{}{}...",
        year,
        seperator,
        letter.unwrap_or_default().to_uppercase()
    );
    let timetable = timetable::timetable(year, args.semester, letter).await;

    println!("Récupération des informations par rapport à l'année...");
    let info = info::info().await;

    if args.export.is_some() {
        // Export the calendar
        let filename = args.export.unwrap();
        println!("Fichier .ICS construit et exporté ici : {}...", filename);

        let builded_timetable = timetable::build(timetable, info);
        ics::export(builded_timetable, filename);
    } else {
        // Show the calendar
        println!("Affichage...");
        timetable::display(timetable);
        println!("Vous devrez peut-être mettre votre terminal en plein écran si ce n'est pas déjà le cas.");
    }
}
