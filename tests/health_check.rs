// use zero2prod::main;

// #[test]
// fn dummy_test() {
//     main()
// }
// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using 
// `cargo expand --test health_check` (<- name of the test file)
use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    // spawn_app().await.expect("Failed to spawn our app.");
    let address = spawn_app();

    // We need to bring in `reqwest` 
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();

    // Act
    let response = client
            // .get("http://127.0.0.1:8000/health_check")
            .get(&format!("{}/health_check", &address))
            .send()
            .await
            .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background ~somehow~
// async fn spawn_app() -> std::io::Result<()> {
fn spawn_app() -> String {
    // zero2prod::run().await
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}