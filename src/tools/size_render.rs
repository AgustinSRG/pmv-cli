// Size rendering

const KB: u64 = 1024;
const MB: u64 = KB * KB;
const GB: u64 = MB * KB;
const TB: u64 = GB * KB;

pub fn render_size_bytes(bytes: u64) -> String {
    if bytes > TB {
        let v = bytes as f64 / TB as f64;
        format!("{v:.2} TB")
    } else if bytes > GB {
        let v = bytes as f64 / GB as f64;
        format!("{v:.2} GB")
    } else if bytes > MB {
        let v = bytes as f64 / MB as f64;
        format!("{v:.2} MB")
    } else if bytes > KB {
        let v = bytes as f64 / KB as f64;
        format!("{v:.2} KB")
    } else {
        format!("{bytes} Bytes")
    }
}
