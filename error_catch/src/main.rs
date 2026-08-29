fn schedule_weather_retry() {
    println!("next wether");
}

fn main() {
    match get_wether(hometown) {
        OK(report) {
            display_weather(hometown, &report);
        }
        Err(err) => {
            println!("error querying the weather: {}", err);
            schedule_weather_retry();
        }
    }
}
