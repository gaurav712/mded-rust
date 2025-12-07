use crate::markdown::chunker::Chunk;
use once_cell::sync::Lazy;
use regex::Regex;

/// Represents a parsed chunk with its HTML output and original index
#[derive(Clone, Debug)]
pub struct ParsedChunk {
    pub html: String,
    pub index: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ListType {
    Unordered,
    Ordered,
}

impl ListType {
    fn as_str(self) -> &'static str {
        match self {
            ListType::Unordered => "ul",
            ListType::Ordered => "ol",
        }
    }
    
    fn open_tag(self, class: Option<&str>) -> String {
        match class {
            Some(c) => format!("<{} class=\"{}\">", self.as_str(), c),
            None => format!("<{}>", self.as_str()),
        }
    }
    
    fn close_tag(self) -> &'static str {
        match self {
            ListType::Unordered => "</ul>",
            ListType::Ordered => "</ol>",
        }
    }
}

/// Regex patterns for markdown parsing
static HEADING_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(#{1,6})\s+(.+)$").expect("Failed to compile heading regex")
});

static INLINE_CODE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"`([^`]+)`").expect("Failed to compile inline code regex")
});

static BOLD_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\*\*([^*]+)\*\*").expect("Failed to compile bold regex")
});

static ITALIC_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"_([^_]+)_").expect("Failed to compile italic regex")
});

static STRIKETHROUGH_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"~~([^~]+)~~").expect("Failed to compile strikethrough regex")
});

static LINK_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").expect("Failed to compile link regex")
});

static IMAGE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"!\[([^\]]*)\]\(([^)]+?)(?:\s+["']([^"']*)["'])?\)"#).expect("Failed to compile image regex")
});

static HORIZONTAL_RULE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^-{3,}$").expect("Failed to compile horizontal rule regex")
});

static TASK_LIST_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(\s*)- \[([ x])\]\s+(.+)$").expect("Failed to compile task list regex")
});

/// Escape HTML special characters
fn escape_html(text: &str) -> String {
    text.chars()
        .fold(String::with_capacity(text.len()), |mut acc, c| {
            match c {
                '<' => acc.push_str("&lt;"),
                '>' => acc.push_str("&gt;"),
                '&' => acc.push_str("&amp;"),
                '"' => acc.push_str("&quot;"),
                '\'' => acc.push_str("&#39;"),
                _ => acc.push(c),
            }
            acc
        })
}

/// Parse inline markdown (bold, italic, links, etc.)
fn parse_inline(text: &str) -> String {
    let mut result = text.to_string();
    
    // Process in order: code (to avoid processing inside code), strikethrough, bold, italic, links, images
    result = INLINE_CODE_REGEX.replace_all(&result, |caps: &regex::Captures| {
        let code = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        format!("<code>{}</code>", escape_html(code))
    }).to_string();
    
    result = STRIKETHROUGH_REGEX.replace_all(&result, |caps: &regex::Captures| {
        let text = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        format!("<del>{}</del>", parse_inline_simple(text))
    }).to_string();
    
    result = BOLD_REGEX.replace_all(&result, |caps: &regex::Captures| {
        let text = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        format!("<strong>{}</strong>", parse_inline_simple(text))
    }).to_string();
    
    result = ITALIC_REGEX.replace_all(&result, |caps: &regex::Captures| {
        let text = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        format!("<em>{}</em>", escape_html(text))
    }).to_string();
    
    result = IMAGE_REGEX.replace_all(&result, |caps: &regex::Captures| {
        let alt = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        let url = caps.get(2).map(|m| m.as_str()).unwrap_or("");
        let title = caps.get(3).map(|m| m.as_str());
        if let Some(title) = title {
            format!(r#"<img src="{}" alt="{}" title="{}" />"#, escape_html(url), escape_html(alt), escape_html(title))
        } else {
            format!(r#"<img src="{}" alt="{}" />"#, escape_html(url), escape_html(alt))
        }
    }).to_string();
    
    result = LINK_REGEX.replace_all(&result, |caps: &regex::Captures| {
        let text = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        let url = caps.get(2).map(|m| m.as_str()).unwrap_or("");
        format!(r#"<a href="{}">{}</a>"#, escape_html(url), parse_inline_simple(text))
    }).to_string();
    
    result
}

/// Parse inline markdown without code (for nested parsing)
fn parse_inline_simple(text: &str) -> String {
    let mut result = text.to_string();
    
    result = BOLD_REGEX.replace_all(&result, |caps: &regex::Captures| {
        let text = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        format!("<strong>{}</strong>", escape_html(text))
    }).to_string();
    
    result = ITALIC_REGEX.replace_all(&result, |caps: &regex::Captures| {
        let text = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        format!("<em>{}</em>", escape_html(text))
    }).to_string();
    
    result
}

/// Parse a table row
fn parse_table_row(line: &str, is_header: bool) -> String {
    let cells: Vec<&str> = line.split('|')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    
    let tag = if is_header { "th" } else { "td" };
    let cell_html: String = cells.iter()
        .map(|cell| format!("<{}>{}</{}>", tag, parse_inline(cell), tag))
        .collect();
    
    format!("<tr>{}</tr>", cell_html)
}

/// Parse a table
fn parse_table(lines: &[&str], start: usize) -> (String, usize) {
    let mut html = String::from("<table><thead>");
    let mut i = start;
    
    // Parse header
    if i < lines.len() {
        html.push_str(&parse_table_row(lines[i], true));
        i += 1;
    }
    
    html.push_str("</thead><tbody>");
    
    // Skip separator line
    if i < lines.len() && HORIZONTAL_RULE_REGEX.is_match(lines[i]) {
        i += 1;
    }
    
    // Parse body rows
    while i < lines.len() {
        let line = lines[i].trim();
        if line.is_empty() || HEADING_REGEX.is_match(line) || line.starts_with("```") {
            break;
        }
        if !HORIZONTAL_RULE_REGEX.is_match(line) && line.contains('|') {
            html.push_str(&parse_table_row(line, false));
        }
        i += 1;
    }
    
    html.push_str("</tbody></table>");
    (html, i)
}

/// Calculate indentation level (number of leading spaces/tabs)
fn get_indent_level(line: &str) -> usize {
    line.chars()
        .take_while(|c| *c == ' ' || *c == '\t')
        .map(|c| if c == '\t' { 4 } else { 1 })
        .sum()
}

/// Check if a line is a list item
fn is_list_item(line: &str) -> bool {
    let trimmed = line.trim_start();
    
    if TASK_LIST_REGEX.is_match(line) {
        return true;
    }
    
    if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
        return true;
    }
    
    if let Some(num_end) = trimmed.find(". ") {
        if trimmed[..num_end].chars().all(|c| c.is_ascii_digit()) {
            return true;
        }
    }
    
    false
}

/// Helper to close current list
fn close_list(html: &mut String, in_list: bool, list_type: ListType) {
    if in_list {
        html.push_str(list_type.close_tag());
    }
}

/// Parse a list (ordered or unordered) with nested list support
fn parse_list(lines: &[&str], start: usize, base_indent: usize) -> (String, usize) {
    let mut html = String::new();
    let mut i = start;
    let mut in_list = false;
    let mut list_type = ListType::Unordered;
    let mut current_item_html = String::new();
    
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();
        
        if trimmed.is_empty() {
            if !current_item_html.is_empty() {
                html.push_str(&current_item_html);
                current_item_html.clear();
            }
            close_list(&mut html, in_list, list_type);
            in_list = false;
            i += 1;
            continue;
        }
        
        let indent = get_indent_level(line);
        
        if indent < base_indent {
            break;
        }
        
        if indent == base_indent {
            if !current_item_html.is_empty() {
                html.push_str(&current_item_html);
                current_item_html.clear();
            }
            
            // Check for task list
            if let Some(caps) = TASK_LIST_REGEX.captures(line) {
                let checked = caps.get(2).map(|m| m.as_str() == "x").unwrap_or(false);
                let text = caps.get(3).map(|m| m.as_str()).unwrap_or("");
                
                if !in_list || list_type != ListType::Unordered {
                    close_list(&mut html, in_list, list_type);
                    html.push_str(&ListType::Unordered.open_tag(Some("contains-task-list")));
                    in_list = true;
                    list_type = ListType::Unordered;
                }
                
                let checked_attr = if checked { " checked" } else { "" };
                current_item_html = format!(
                    "<li class=\"task-list-item\"><input type=\"checkbox\" class=\"task-list-item-checkbox\"{} disabled> {}</li>",
                    checked_attr,
                    parse_inline(text)
                );
                i += 1;
                continue;
            }
            
            // Check for unordered list
            if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
                if !in_list || list_type != ListType::Unordered {
                    close_list(&mut html, in_list, list_type);
                    html.push_str(&ListType::Unordered.open_tag(None));
                    in_list = true;
                    list_type = ListType::Unordered;
                }
                let text = &trimmed[2..];
                current_item_html = format!("<li>{}", parse_inline(text));
                i += 1;
                continue;
            }
            
            // Check for ordered list
            if let Some(num_end) = trimmed.find(". ") {
                if trimmed[..num_end].chars().all(|c| c.is_ascii_digit()) {
                    if !in_list || list_type != ListType::Ordered {
                        close_list(&mut html, in_list, list_type);
                        html.push_str(&ListType::Ordered.open_tag(None));
                        in_list = true;
                        list_type = ListType::Ordered;
                    }
                    let text = &trimmed[num_end + 2..];
                    current_item_html = format!("<li>{}", parse_inline(text));
                    i += 1;
                    continue;
                }
            }
            
            break;
        } else if indent > base_indent {
            let trimmed_line = line.trim_start();
            if is_list_item(trimmed_line) {
                if !current_item_html.is_empty() {
                    let (nested_html, new_i) = parse_list(lines, i, indent);
                    current_item_html.push_str(&nested_html);
                    current_item_html.push_str("</li>");
                    html.push_str(&current_item_html);
                    current_item_html.clear();
                    i = new_i;
                    continue;
                } else {
                    let (nested_html, new_i) = parse_list(lines, i, indent);
                    html.push_str(&nested_html);
                    i = new_i;
                    continue;
                }
            } else {
                if !current_item_html.is_empty() && !current_item_html.ends_with("</li>") {
                    let text = line.trim();
                    if !text.is_empty() {
                        current_item_html.push(' ');
                        current_item_html.push_str(&parse_inline(text));
                    }
                }
            }
            i += 1;
        } else {
            break;
        }
    }
    
    if !current_item_html.is_empty() {
        if !current_item_html.ends_with("</li>") {
            current_item_html.push_str("</li>");
        }
        html.push_str(&current_item_html);
    }
    
    close_list(&mut html, in_list, list_type);
    
    (html, i)
}

/// Parse a blockquote
fn parse_blockquote(lines: &[&str], start: usize) -> (String, usize) {
    let mut html = String::from("<blockquote>");
    let mut i = start;
    let mut in_quote = false;
    
    while i < lines.len() {
        let line = lines[i];
        if let Some(pos) = line.find("> ") {
            if in_quote {
                html.push_str("<br>");
            }
            let text = &line[pos + 2..];
            html.push_str(&format!("<p>{}</p>", parse_inline(text)));
            in_quote = true;
            i += 1;
        } else if let Some(pos) = line.find('>') {
            if in_quote {
                html.push_str("<br>");
            }
            let text = line[pos + 1..].trim_start();
            if !text.is_empty() {
                html.push_str(&format!("<p>{}</p>", parse_inline(text)));
            }
            in_quote = true;
            i += 1;
        } else if line.trim().is_empty() && in_quote {
            i += 1;
            continue;
        } else {
            break;
        }
    }
    
    html.push_str("</blockquote>");
    (html, i)
}

/// Parse a code block
fn parse_code_block(lines: &[&str], start: usize) -> (String, usize) {
    let mut i = start;
    let first_line = lines[i].trim();
    
    let lang = first_line.strip_prefix("```")
        .and_then(|s| {
            let trimmed = s.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        });
    
    i += 1;
    let mut code_lines = Vec::new();
    
    while i < lines.len() {
        let line = lines[i];
        if line.trim() == "```" {
            i += 1;
            break;
        }
        code_lines.push(line);
        i += 1;
    }
    
    let code = code_lines.join("\n");
    let lang_attr = lang.map(|l| format!(r#" class="language-{}""#, escape_html(l))).unwrap_or_default();
    let html = format!("<pre><code{}>{}</code></pre>", lang_attr, escape_html(&code));
    
    (html, i)
}

/// Parse a single markdown chunk to HTML
pub fn parse_chunk(chunk: Chunk) -> ParsedChunk {
    let lines: Vec<&str> = chunk.content.lines().collect();
    let mut html = String::new();
    let mut i = 0;
    
    while i < lines.len() {
        let line = lines[i].trim();
        
        if line.is_empty() {
            i += 1;
            continue;
        }
        
        if line.starts_with("```") {
            let (code_html, new_i) = parse_code_block(&lines, i);
            html.push_str(&code_html);
            i = new_i;
            continue;
        }
        
        if HORIZONTAL_RULE_REGEX.is_match(line) {
            html.push_str("<hr>");
            i += 1;
            continue;
        }
        
        if let Some(caps) = HEADING_REGEX.captures(line) {
            let level = caps.get(1).map(|m| m.as_str().len()).unwrap_or(1);
            let text = caps.get(2).map(|m| m.as_str()).unwrap_or("");
            html.push_str(&format!("<h{}>{}</h{}>", level, parse_inline(text), level));
            i += 1;
            continue;
        }
        
        if line.contains('|') && i + 1 < lines.len() && lines[i + 1].trim().contains("---") {
            let (table_html, new_i) = parse_table(&lines, i);
            html.push_str(&table_html);
            i = new_i;
            continue;
        }
        
        if line.starts_with('>') {
            let (quote_html, new_i) = parse_blockquote(&lines, i);
            html.push_str(&quote_html);
            i = new_i;
            continue;
        }
        
        if is_list_item(line) || TASK_LIST_REGEX.is_match(&lines[i]) {
            let base_indent = get_indent_level(&lines[i]);
            let (list_html, new_i) = parse_list(&lines, i, base_indent);
            html.push_str(&list_html);
            i = new_i;
            continue;
        }
        
        // Regular paragraph
        let mut para_lines = Vec::new();
        while i < lines.len() {
            let current_line = lines[i].trim();
            if current_line.is_empty() {
                break;
            }
            if HEADING_REGEX.is_match(current_line) ||
               current_line.starts_with("```") ||
               current_line.starts_with('>') ||
               is_list_item(current_line) ||
               HORIZONTAL_RULE_REGEX.is_match(current_line) ||
               (current_line.contains('|') && i + 1 < lines.len() && lines[i + 1].trim().contains("---")) {
                break;
            }
            para_lines.push(current_line);
            i += 1;
        }
        
        if !para_lines.is_empty() {
            let para_text = para_lines.join(" ");
            html.push_str(&format!("<p>{}</p>", parse_inline(&para_text)));
        }
    }
    
    ParsedChunk {
        html,
        index: chunk.index,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_chunk_basic() {
        let chunk = Chunk {
            content: "# Hello World".to_string(),
            index: 0,
        };
        let parsed = parse_chunk(chunk);
        assert!(parsed.html.contains("<h1>"));
        assert_eq!(parsed.index, 0);
    }

    #[test]
    fn test_parse_chunk_gfm_features() {
        let chunk = Chunk {
            content: "- [x] Task\n~~strikethrough~~".to_string(),
            index: 0,
        };
        let parsed = parse_chunk(chunk);
        assert!(parsed.html.contains("task-list-item"));
        assert!(parsed.html.contains("strikethrough") || parsed.html.contains("<del>"));
    }
}
