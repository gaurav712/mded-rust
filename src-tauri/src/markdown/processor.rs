use crate::markdown::chunker::chunk_by_lines;
use crate::markdown::highlighter::highlight_code_blocks;
use crate::markdown::parser::{parse_chunk, ParsedChunk};
use rayon::prelude::*;

/// Default chunk size (lines per chunk)
const DEFAULT_CHUNK_SIZE: usize = 500;

/// Process markdown in parallel chunks and return HTML
/// 
/// # Arguments
/// * `markdown` - The markdown content to process
/// * `num_threads` - Optional number of threads to use (None uses default)
/// 
/// # Returns
/// HTML string with syntax highlighted code blocks
pub fn process_markdown_parallel(markdown: &str, num_threads: Option<usize>) -> String {
    // Set thread pool size if specified
    if let Some(threads) = num_threads {
        let _ = rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build_global();
    }

    // Split markdown into chunks
    let chunks = chunk_by_lines(markdown, DEFAULT_CHUNK_SIZE);

    // If only one chunk, process directly without parallel overhead
    if chunks.len() == 1 {
        let parsed = parse_chunk(chunks.into_iter().next().unwrap());
        return highlight_code_blocks(&parsed.html);
    }

    // Process chunks in parallel
    let parsed_chunks: Vec<ParsedChunk> = chunks
        .into_par_iter()
        .map(parse_chunk)
        .collect();

    // Sort by index to maintain original order
    let mut sorted_chunks = parsed_chunks;
    sorted_chunks.sort_by_key(|chunk| chunk.index);

    // Combine HTML from all chunks
    let combined_html: String = sorted_chunks
        .into_iter()
        .map(|chunk| chunk.html)
        .fold(String::new(), |mut acc, html| {
            if !acc.is_empty() {
                acc.push('\n');
            }
            acc.push_str(&html);
            acc
        });

    // Apply syntax highlighting to the combined HTML
    highlight_code_blocks(&combined_html)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_markdown_parallel_basic() {
        let markdown = "# Hello\n\nWorld";
        let html = process_markdown_parallel(markdown, None);
        assert!(html.contains("<h1>"));
    }

    #[test]
    fn test_process_markdown_parallel_preserves_order() {
        let markdown = "# First\n\n# Second\n\n# Third";
        let html = process_markdown_parallel(markdown, None);
        let first_pos = html.find("First");
        let second_pos = html.find("Second");
        let third_pos = html.find("Third");
        assert!(first_pos < second_pos);
        assert!(second_pos < third_pos);
    }

    #[test]
    fn test_process_markdown_parallel_with_code() {
        let markdown = "```js\nconsole.log('test');\n```";
        let html = process_markdown_parallel(markdown, None);
        assert!(html.contains("language-js"));
    }
}
