
**ghost.ai** is a _"second brain"_ that helps you remember everything you see and do on your computer.  
> blindly influenced by [Rauch’s post](https://x.com/rauchg/status/1903528336241861113)

It supports you in two main ways: **writing** and **recall**.

For example:

> You’re trying to finish a sentence using something you just read.  
> Or you ask:  
> _"I read a post from Rauch on X about the best technologies humans have invented... but I don't remember exactly what it said."_

ghost.ai can remind you — based on what was on your screen at that time.

Or maybe you’re typing a message after reading an article.  
ghost.ai can recognize the pattern and **autocomplete** your sentence — using the content you just saw.




## Status

**This is an early prototype.**

- The first version (`prototype-v1`) only tracked keyboard input.  
- This version adds a **vision system**:
  - Captures screen frames  
  - Extracts text using OCR  
  - Stores the result in memory for future use  

Note: This is experimental software.  
It does **not** yet ignore sensitive windows like password fields or private messages.




## Compatibility

- Tested on **macOS** only  
- Should work on most Unix-based systems (may need small changes)  
- OCR currently supports **English only**




## Features

- Screen capture + OCR (Vision)
- Text memory with embeddings
- Terminal chat to ask: _"What did I see before?"_
- Keyboard + mouse tracking
- Simple integration with [Ollama](https://ollama.com/) for local LLMs




## Installation

Clone the repo:

```bash
git clone https://github.com/yazaldefilimone/ghost.ai.git
cd ghost.ai
```

---

### 1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```




### 2. Download OCR models

ghost.ai uses neural networks for OCR.  
Run this script:

```bash
./scripts/download-models.sh
```

This will download:

- `text-detection.rten`  
- `text-recognition.rten`  

They will be saved in the `./models/` folder.




### 3. Install and setup Ollama

Make sure [Ollama](https://ollama.com/) is installed.

By default, ghost.ai uses:

```bash
ollama run mistral:7b-instruct
```

You can change the model in `src/llm.rs` (`const MODEL`).




## Running

1. allow app(terminal ou IDE) to control your computer(include record screen)

![CleanShot 2025-04-14 at 14 11 39@2x](https://github.com/user-attachments/assets/1fb50806-af62-4a44-9679-5ebbf6378556)


Start the system:

```bash
cargo run
```

ghost.ai will begin monitoring your screen.

- On an M3 Mac with 16 GB RAM, each frame takes about **10 seconds** to process.




## Usage

1. Open any window (browser, PDF, terminal, etc.)
2. ghost.ai captures what’s visible
3. Later, you can ask:  
   _"What did I see when I opened the article about X?"_
4. While typing, ghost.ai may suggest **autocompletions** based on your recent screen content



## Roadmap

- Support for cloud LLMs (OpenAI, Claude, etc.)
- Automatically skip sensitive/private windows
- Faster screen capture + batch processing
- Audio capture (with on/off toggle)
- Local web chat interface
- Multilingual OCR support
