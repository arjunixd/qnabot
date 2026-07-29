# Q&A Bot

A **Retrieval-Augmented Generation (RAG) bot** built with Rust and [Rig](https://docs.rs/rig-core) framework. It ingests Markdown documents splits them into token‑aware chunks, stores embeddings in memory and answers questions via web UI.

---

## Features

- **Token‑aware chunking** – uses `chunkedrs` to split Markdown while respecting token limits (512 tokens) with configurable overlap.
- **Local embeddings** – FastEmbed (`AllMiniLML6V2Q`) for fast, offline vectorisation.
- **HuggingFace LLM** – powered by `google/gemma-4-26B-A4B-it:fastest` (easily swappable).
- **Web UI & REST API** – interactive chat interface with `/ask` and `/health` endpoints.

---

## Project Structure

```
qa-bot/
├── Cargo.toml
├── env.example
├── dataset/               # Place your Markdown files here
│   └── qna.md
├── static/                # Web UI assets
│   ├── index.html
│   ├── style.css
│   └── app.js
└── src/
    ├── main.rs         
    ├── lib.rs            
    ├── agent.rs          
    ├── chunking.rs       
    └── routes.rs         
```

## Quick Start

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable)
- A [HuggingFace API key](https://huggingface.co/settings/tokens) (free tier works)

### Installation
```bash
git clone https://github.com/arjunixd/qnabot.git
cd qnabot
cp env.example .env
# Add your HUGGINGFACE_API_KEY to .env
```

Place your Markdown documents in `dataset/qna.md` (or change the path in `main.rs`).

### Run
```bash
cargo run
```
Open `http://localhost:3000` in your browser.

---

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET`  | `/`       | Serves the web UI (static files) |
| `GET`  | `/health` | Health check – returns `"OK"` |
| `POST` | `/ask`    | Accepts `{ "question": "..." }`; returns `{ "answer": "..." }` |

---

## TODO

### Phase 1 – Core Functionality (Done)
- [x] Token‑aware chunking with overlap
- [x] In‑memory vector store
- [x] API integration/HuggingFace
- [x] Web UI & REST API

### Phase 2 – Persistence & Scalability
- [ ] **Replace in‑memory store** with a persistent vector database (e.g. **LanceDB**, **Qdrant**, **PostgreSQL+pgvector** or `rig-sqlite`).
- [ ] **Separate indexing pipeline** – avoid re‑embedding at every restart; index files asynchronously.
- [ ] **Hybrid search** – combine dense embeddings with BM25 keyword search for better recall.
- [ ] **Re‑ranking** – apply a cross‑encoder (e.g., `ms-marco-MiniLM-L-6-v2`) to refine the top‑K retrieved chunks.

### Phase 3 – Performance & Caching
- [ ] **Semantic caching** – cache embeddings and LLM responses for frequent queries (using Redis).
- [ ] **Response caching** – store exact question‑answer pairs with TTL to reduce latency.
- [ ] **Prompt caching** – reuse static parts of the prompt to reduce token usage.
- [ ] **Query rewrite cache** – avoid re‑rewriting similar questions.

---

## License
This project is licensed under the MIT License. See [LICENSE](https://github.com/arjunixd/qnabot/blob/main/LICENSE) for details.
