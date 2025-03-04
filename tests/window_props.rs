// Compares bucketed windows with an exact reference on random event streams.
// Recorded: up to 9% error on the 1 minute window at bucket edges (see results/).
#[test]
fn bucket_error_is_bounded_on_random_streams() {
    let max_error = feature_store::window::buckets::max_edge_error(10_000, 42);
    assert!(max_error < 0.10, "edge error {max_error}");
}
