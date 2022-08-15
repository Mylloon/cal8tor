mod info;
mod timetable;

#[tokio::main]
async fn main() {
    let _timetable = timetable::timetable(3, 1, None).await;

    let _info = info::info().await;
}
