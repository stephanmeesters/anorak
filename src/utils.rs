use chrono::DateTime;
use log::info;

pub fn format_bytes(bytes: u64) -> String {
    let sizes = ["Bytes", "KB", "MB", "GB", "TB"];
    if bytes == 0 {
        return "0 Bytes".to_string();
    }
    let i = (bytes as f64).log(1024.0).floor() as usize;
    if i == 0 {
        return format!("{} {}", bytes, sizes[i]);
    }
    let readable_size = bytes as f64 / 1024_f64.powi(i as i32);
    format!("{:.2} {}", readable_size, sizes[i])
}

pub fn format_date(date_str: &str) -> String {
    let date = DateTime::parse_from_str(date_str, "%a, %d %b %Y %H:%M:%S %z");
    match date {
        Ok(date_time) => date_time.format("%a, %d %b %Y").to_string(),
        Err(_e) => date_str.to_owned()
    }
}

fn print_request(request: &reqwest::Request, body: &str) {
    info!(
        "Request\n\tMethod: {}\n\tURL: {}\n\tHeaders:{}\n\tBody: {}",
        request.method(),
        request.url(),
        request
            .headers()
            .iter()
            .map(|(key, value)| format!("{}: {:?}", key, value))
            .collect::<Vec<String>>()
            .join("\n"),
        body
    );
}
