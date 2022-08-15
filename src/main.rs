mod info;
mod io;
mod timetable;

#[tokio::main]
async fn main() {
    let timetable = timetable::timetable(3, 1, None).await;

    let info = info::info().await;

    io::export(timetable, info);
}
