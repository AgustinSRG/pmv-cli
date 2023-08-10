// Size rendering

pub fn render_size_bytes(bytes: u64) -> String {
    if bytes > 1024 * 1024 * 1024 {
        let v = bytes as f64 / (1024 * 1024 * 1024) as f64;
        format!("{v:.2} GB")
    } else if bytes > 1024 * 1024 {
        let v = bytes as f64 / (1024 * 1024) as f64;
        format!("{v:.2} MB")
    } else if bytes > 1024 {
        let v = bytes as f64 / 1024_f64;
        format!("{v:.2} KB")
    } else {
        format!("{bytes} Bytes")
    }
}
