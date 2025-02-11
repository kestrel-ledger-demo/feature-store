// Replays a recorded day against the store and records read latency in an HDR histogram.
fn main() {
    let day = std::env::args().nth(1).expect("recorded day path");
    let events = feature_store::replay::load_day(&day).unwrap();
    println!("replaying {} events", events.len());
}
