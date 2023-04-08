
mod censor;

// Python reference
pub fn handle_command(cmd: &str, kwargs: Vec<&str>) -> String {
    let args = kwargs.iter().map(|&v| v.to_owned()).collect();
    match cmd {
        "/censor" => censor::execute(args),
        &_ => "".to_owned()
    }
}
