use crate::chunking;
use anyhow::Result;
use rig::{
    client::{CompletionClient, ProviderClient},
    embeddings::EmbeddingsBuilder,
    fastembed,
    providers::huggingface::{self, completion::CompletionModel},
    vector_store::in_memory_store::InMemoryVectorStore,
};

const MAX_TOKENS: usize = 512;
const OVERLAP: usize = 50;

pub type Agent = rig::agent::Agent<CompletionModel>;

pub async fn build_agent(dataset_path: &str) -> Result<Agent> {
    let chunks = chunking::load_chunks(dataset_path, MAX_TOKENS, OVERLAP)?;

    let emb_model =
        fastembed::Client::new().embedding_model(&fastembed::FastembedModel::AllMiniLML6V2Q)?;

    let embeddings = EmbeddingsBuilder::new(emb_model.clone())
        .documents(chunks)?
        .build()
        .await?;

    let store = InMemoryVectorStore::from_documents(embeddings);
    let index = store.index(emb_model);

    let hf = huggingface::Client::from_env()?;

    Ok(hf
        .agent("google/gemma-4-26B-A4B-it:fastest")
        .preamble(
            "You are a precise Q&A assistant. Use the provided context to answer the user question. \
             If the context does not contain the answer, say 'I don't have that information.'",
        )
        .dynamic_context(3, index)
        .build())
}
