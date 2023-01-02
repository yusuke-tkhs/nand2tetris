mod jack;
mod tokens;

pub use jack::class_to_xml;
pub use tokens::tokens_to_xml;

fn sanitize(s: String) -> String {
    s.chars()
        .map(|c| match c {
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '&' => "&amp;".to_string(),
            _ => c.to_string(),
        })
        .collect()
}
