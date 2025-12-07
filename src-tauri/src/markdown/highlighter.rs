/// Apply syntax highlighting to all code blocks in HTML
/// Since we're not using syntect, this just returns the HTML as-is
/// 
/// # Arguments
/// * `html` - HTML string containing code blocks
/// 
/// # Returns
/// HTML with properly formatted code blocks
pub fn highlight_code_blocks(html: &str) -> String {
    // Code blocks are already properly formatted by the parser
    html.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_code_blocks() {
        let html = r#"<pre><code class="language-js">console.log('test');</code></pre>"#;
        let result = highlight_code_blocks(html);
        assert_eq!(result, html);
    }
}
