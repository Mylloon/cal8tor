mod ics;
mod info;
mod timetable;
mod utils;

#[tokio::main]
async fn main() {
    println!("Fetch the timetable...");
    let timetable = timetable::timetable(3, 1, None).await;

    println!("Fetch informations about the year...");
    let info = info::info().await;

    println!("Build the ICS file...");
    let builded_timetable = timetable::build(timetable, info);

    ics::export(builded_timetable, "target/debug.ics");
}
