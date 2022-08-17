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

    println!(
        "Fetch the timetable for L{}{}...",
        year,
        letter.unwrap_or_default()
    );
    let timetable = timetable::timetable(year, args.semester, letter).await;

    println!("Fetch informations about the year...");
    let info = info::info().await;

    if args.export.is_some() {
        // Export the calendar
        let filename = args.export.unwrap();
        println!("Build the ICS file at {}...", filename);

        let builded_timetable = timetable::build(timetable, info);
        ics::export(builded_timetable, filename);
    } else {
        // Show the calendar
        println!("Displaying...");
        timetable::display();
    }
}
