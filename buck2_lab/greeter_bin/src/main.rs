fn main() {
    logging_lib::info("Starting...");

    let message = greeter_lib::greet("Buck2");
    println!("{}", message);

    logging_lib::info("Exit.");
}
