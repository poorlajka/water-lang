use crate::Diagnostic;
use crate::Label;


pub fn emit_diagnostics(code: &str, diagnostics: &[Diagnostic]) {
    let line_index = build_line_index(code);
    for diagnostic in diagnostics {
        println!("{}: {}", diagnostic.severity, diagnostic.message);
        for label in &diagnostic.labels {
            let (line_start, col_start)  = byte_to_line_col(&line_index, label.span.start);
            let (line_end, col_end)  = byte_to_line_col(&line_index, label.span.end);
            println!(" --> {}:{}:{}", "filename.wtr", line_start, col_start);
            emit_label(code, &line_index, label);
        }
    }
}

fn build_line_index(src: &str) -> Vec<usize> {
    let mut lines = vec![0];

    for (i, c) in src.char_indices() {
        if c == '\n' {
            lines.push(i + 1);
        }
    }

    lines
}

fn byte_to_line_col(lines: &[usize], byte: usize) -> (usize, usize) {
    let line = match lines.binary_search(&byte) {
        Ok(i) => i,
        Err(i) => i - 1,
    };

    let col = byte - lines[line] + 1;
    (line + 1, col)
}

fn emit_label(src: &str, lines: &[usize], label: &Label) {
    let start = label.span.start;
    let end = label.span.end;

    // Map start and end to (line, col)
    let (start_line, start_col) = byte_to_line_col(lines, start);
    let (end_line, end_col) = byte_to_line_col(lines, end);

    // Only handle single-line spans for now
    let line_start_byte = lines[start_line - 1];
    let line_end_byte = lines
        .get(start_line)
        .copied()
        .unwrap_or(src.len());

    let line_text = &src[line_start_byte..line_end_byte];

    println!("{} | {}", start_line, line_text);

    // Marker line
    let mut marker = String::new();
    for _ in 0..(start_col - 1) {
        marker.push(' ');
    }
    if end > start {
        for _ in start_col - 1..end_col - 1 {
            marker.push('^'); // Could use '~' for multi-char spans
        }
    } else {
        marker.push('^');
    }

    if let Some(msg) = &label.message {
        marker.push_str(" ");
        marker.push_str(msg);
    }

    println!("   | {}", marker);
}