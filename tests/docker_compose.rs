#[test]
fn compose_services_rebuild_images_before_running() {
    let compose = std::fs::read_to_string("docker-compose.yml").unwrap();

    assert_eq!(
        compose.matches("pull_policy: build").count(),
        2,
        "both the test and runtime services should force rebuilds so runs cannot silently use stale local images"
    );
}

#[test]
fn compose_images_use_the_release_version() {
    let compose = std::fs::read_to_string("docker-compose.yml").unwrap();

    let image_lines = compose.lines().map(str::trim).collect::<Vec<_>>();
    assert!(image_lines.contains(&"image: cust:v0.56.0"));
    assert!(image_lines.contains(&"image: cust-test:v0.56.0"));
}
