use clap::Parser;

mod ics;
mod info;
mod timetable;
mod utils;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Class
    #[clap(short, long, value_parser)]
    class: i8,

    /// Subgroup if you have one, for example in `L1-A`, specify here the `A`
    #[clap(short, long, value_parser)]
    letter: Option<char>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("class: L{}{}", args.class, args.letter.unwrap_or_default());

    /* println!("Fetch the timetable...");
    let timetable = timetable::timetable(3, 1, None).await;

    println!("Fetch informations about the year...");
    let info = info::info().await;

    println!("Build the ICS file...");
    let builded_timetable = timetable::build(timetable, info);

    ics::export(builded_timetable, "target/debug.ics"); */
}
