mod ics;
mod info;
mod timetable;

#[tokio::main]
async fn main() {
    let timetable = timetable::timetable(3, 1, None).await;

    let info = info::info().await;

    let builded_timetable = timetable::build(timetable, info);

    ics::export(builded_timetable, "target/debug.ics");
}
