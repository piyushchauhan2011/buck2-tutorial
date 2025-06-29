pub fn greet(name: &str) -> String {
    // Let's use our new logging library!
    logging_lib::info("Entered greet function in library");

    let greeting = format!("Hello, {}!", name);

    logging_lib::info("Exiting greet function in library");
    greeting
}
