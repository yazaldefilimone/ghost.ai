
ghost.ai is a "second brain" that helps you remember everything you see and do on your computer. 
> blindly influenced by [Rauch's post](https://x.com/rauchg/status/1903528336241861113).

It assists you during both **writing** and **recall** — whether you're trying to finish a sentence using something you just read, or asking:

> "I read a post from Rauch on X about the best technologies humans have invented... but I don't remember exactly what it said."

ghost.ai can remind you — based on what was on your screen at the time.

or maybe you're typing a message after reading an article — ghost.ai can recognize the pattern and **autocomplete** your sentence based on the article you just saw.


## Status

**this is an early prototype.**

the first version (see branch `prototype-v1`) only listened to keyboard input.

this version introduces a **vision system** that captures screen frames, extracts text using OCR, and stores the result in memory as context for later use.

NB: This is experimental software. It does **not** yet ignore sensitive windows like password fields or private messages.

---

## compatibility

- tested on **macOS** only.  
- should work on most Unix-based systems, but may require minor adjustments.  
- OCR models currently support **english only**.

---

## Features

- screen capture and OCR (vision)
- text memory with embeddings
- terminal chat to ask what you've seen before
- keyboard and mouse tracking (tracker)
- simple integration with [Ollama](https://ollama.com/) for local LLM inference

---

## Installation

```bash
git clone https://github.com/yazaldefilimone/ghost.ai.git
cd ghost.ai
```

### 1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. download OCR models

ghost.ai uses neural networks for OCR. Run the script:

```bash
./scripts/download-models.sh
```

This will download:

- `text-detection.rten`
- `text-recognition.rten`

These are placed in the `./models/` directory.

### 3. Install and Setup Ollama

make sure [Ollama](https://ollama.com/) is installed.

the default LLM is:

```bash
ollama run mistral:7b-instruct
```

You can change the model in `src/llm.rs` (`const MODEL`).

---

## Running

```bash
cargo run
```

after launch, ghost.ai will start monitoring your screen.

- on an M3 Mac with 16 GB RAM, each frame takes ~10 seconds to process.

---

## Usage

- open any window or document (like a browser, PDF, or terminal)
- ghost.ai captures and stores what you see
- you can ask the system later:  
  *"What did I see when I opened the article about X?"*  
- you can also get **autocomplete suggestions** while typing, based on that context

---

## Plans

- support for cloud LLMs (OpenAI, Claude, etc.)
- filter sensitive/private windows automatically
- faster screen capture + batch inference
- audio capture — with toggle to disable vision/audio
- local web chat interface
- multilingual OCR support
---
