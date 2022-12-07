pub fn read_file() -> Result<String, String> {
    let mut args = std::env::args();

    let file_name = args
        .nth(1)
        .ok_or_else(|| String::from("Please provide file path."))?;

    std::fs::read_to_string(&file_name).map_err(|_| format!("Could not open file {file_name}"))
}
