/// Represents a chunk of markdown content with its index for ordering
#[derive(Clone, Debug)]
pub struct Chunk {
    pub content: String,
    pub index: usize,
}

/// Split markdown into line-based chunks for parallel processing
/// 
/// # Arguments
/// * `markdown` - The markdown content to split
/// * `chunk_size` - Number of lines per chunk
/// 
/// # Returns
/// Vector of chunks with preserved order indices
pub fn chunk_by_lines(markdown: &str, chunk_size: usize) -> Vec<Chunk> {
    if markdown.is_empty() {
        return vec![Chunk {
            content: String::new(),
            index: 0,
        }];
    }

    let lines: Vec<&str> = markdown.lines().collect();
    let total_lines = lines.len();
    
    if total_lines == 0 {
        return vec![Chunk {
            content: String::new(),
            index: 0,
        }];
    }

    let mut chunks = Vec::new();
    let mut chunk_index = 0;

    for chunk_lines in lines.chunks(chunk_size) {
        let content = chunk_lines.join("\n");
        chunks.push(Chunk {
            content,
            index: chunk_index,
        });
        chunk_index += 1;
    }

    chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_by_lines_empty() {
        let chunks = chunk_by_lines("", 100);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].content, "");
        assert_eq!(chunks[0].index, 0);
    }

    #[test]
    fn test_chunk_by_lines_single_chunk() {
        let markdown = "Line 1\nLine 2\nLine 3";
        let chunks = chunk_by_lines(markdown, 100);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].content, markdown);
        assert_eq!(chunks[0].index, 0);
    }

    #[test]
    fn test_chunk_by_lines_multiple_chunks() {
        let markdown = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";
        let chunks = chunk_by_lines(markdown, 2);
        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0].content, "Line 1\nLine 2");
        assert_eq!(chunks[0].index, 0);
        assert_eq!(chunks[1].content, "Line 3\nLine 4");
        assert_eq!(chunks[1].index, 1);
        assert_eq!(chunks[2].content, "Line 5");
        assert_eq!(chunks[2].index, 2);
    }
}
