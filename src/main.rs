fn extract_severity(log: &str) -> &str {
    if let Some(start) = log.find('['){
        // +1 skips the '[' (one byte). `rest` is still just a view into `log`
        let rest = &log[start + 1..];
        // Relative to `rest`, not `log`, so slice `rest` below
        if let Some(end) = rest.find(']') {
            return &rest[..end];
        }
    }
    // Only reached if a bracket was missing
    log
}

fn append_alert(summary: &mut String, severity: &str, message: &str) {
    summary.push_str("[");
    summary.push_str(severity);
    summary.push_str("] ");
    summary.push_str(message);
    summary.push_str("\n");
}

fn archive_log(mut log: String, code: u32) -> String {
    log.push_str(" [CODE: ");
    // `code` is a u32 but `push_str` only takes text so 404 becomes "404" first
    log.push_str(&code.to_string());
    log.push_str("]");
    log // No `;` so ownership moves back out tot he caller
}

fn main() {
    let raw_log = String::from("[WARN] Disk space reaching 90%");
    let status: u32 = 404;

    let severity = extract_severity(&raw_log);
    println!("Extracted Severity: {severity}");

    let mut summary = String::new();
    append_alert(&mut summary, severity, "Disk space low");
    println!("\nSystem Log Summary:\n{summary}");

    // Ordering matters: this moves `raw_log` into archive_log.
    // `severity` borrows from `raw_log` so it must be finished before this line.
    // Results in a compile error (E0505) if used after.
    let archived = archive_log(raw_log, status);
    println!("Archived output: {archived}");
}