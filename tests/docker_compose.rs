#[test]
fn compose_services_rebuild_images_before_running() {
    let compose = std::fs::read_to_string("docker-compose.yml").unwrap();

    assert_eq!(
        compose.matches("pull_policy: build").count(),
        2,
        "both the test and runtime services should force rebuilds so runs cannot silently use stale local images"
    );
}
