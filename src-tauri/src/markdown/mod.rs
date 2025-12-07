pub mod chunker;
pub mod parser;
pub mod highlighter;
pub mod processor;

pub use processor::process_markdown_parallel;
