use anyhow::{Context, Result};
use chunkedrs::Chunk;

pub fn load_chunks(path: &str, max_tokens: usize, overlap: usize) -> Result<Vec<String>> {
    let text = std::fs::read_to_string(path).with_context(|| format!("Could not read {}", path))?;
    let chunks: Vec<Chunk> = chunkedrs::chunk(&text)
        .markdown()
        .max_tokens(max_tokens)
        .overlap(overlap)
        .model("gpt-4o")
        .split();
    Ok(chunks.into_iter().map(|c| c.content).collect())
}
