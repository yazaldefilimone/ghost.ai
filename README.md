# Ghost

A second brain for your computer: vision, hear, memory, smart writing — all locally by default.

## Index

1. [Introduction](#introduction)
2. [Important Notes](#important-notes)
3. [Install](#install)
4. [Getting Started](#getting-started)
5. [Usage](#usage)
6. [Configuration](#configuration)
7. [Roadmap](#roadmap)
8. [Additional Resources](#additional-resources)

## Introduction

Ghost is a lightweight, local-first _second brain_ that helps you remember everything you see and do on your computer.
It captures screen frames, extracts visible text using OCR, stores the information, and lets you recall, autocomplete, or **chat** based on your visual memory.

Ghost supports three main flows:

- **Recall**: _"What did I see when I opened X?"_
- **Writing Support**: Autocomplete sentences based on recent screen context.
- **Memory Chat**: A built-in chat where you can **talk with your memories**, like a ChatGPT trained only on what you saw.

Ghost is modular and highly configurable — each memory stage (vision, chat, autocomplete, hearing) can be powered by different models, locally or remotely.

> Ghost is blindly influenced by [Guillermo Rauch's vision](https://x.com/rauchg/status/1903528336241861113), but built with full offline privacy in mind.

## Important Notes

- Ghost is still in **bate** stage.
- Only tested on **macOS**. Other Unix systems should work with small tweaks.
- OCR currently supports **English only**.
- Sensitive data handling: supports **skip patterns** and **skip app filters**.
- The **audio system for meetings and calls** (`hear`) is **under active development** and will arrive in future versions.
- Ghost includes a **terminal chat interface** to interact with your memories like a private ChatGPT.
- Local LLMs are recommended (default: [Ollama](https://ollama.com/)).
- All data are store at `.config/ghost/.data.db`

## Install

### Install dependencies

```sh

# Install Rust if you haven't already.
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Install Ghost

You can install Ghost via Cargo:

```sh
cargo install ghostai@0.2.1-beta
```

### Setup vision and settings

```sh

ghost init
```

### Install and configure Ollama

Ghost uses [Ollama](https://ollama.com/) by default to run local LLMs.
Make sure Ollama is installed and running.

Default model:

```sh

ollama run mistral:7b-instruct
```

Model selection is fully customizable per stage.

## Getting Started

### Permissions

Allow your terminal or IDE to:

- Control your computer
- Record your screen

(System Preferences → Privacy & Security → Screen Recording)

### Running Ghost

```sh

ghost run
```

Ghost will start capturing frames, extracting text, building memory, and listening for input.

## Usage

- Open any window (browser, terminal, PDF, editor, etc.).
- Ghost captures visible content automatically.
- While typing, you can use shortcuts to ghost may suggest completions based on what you just saw.
- You can chat with your memory via the terminal:

```sh

chat >> What was the article about the best technologies humans have invented?
```

Ghost acts like a **private GPT** that only knows what you have personally seen — not generic internet data.

## Configuration

Ghost is **deeply customizable**.

Example configuration (`.config/ghost/settings.toml`):

```toml

name = "Your Name"
language = "en"

[embed]
provider = "ollama"
model = "nomic-embed-text"

[autocomplete]
provider = "ollama"
model = "mistral:7b-instruct"
stream = true
skip_app = ["code", "zed"]
discard_behavior = "full"

[chat]
enabled = true
provider = "ollama"
model = "mistral:7b-instruct"

[hear]
enabled = false
model = "whisper" # or elevenlabs

[vision]
skip_app = ["code", "zed"]
skip_patterns = []
security_skip = true

[shortcuts.macos]
look = "cmd+c"
autocomplete = "cmd+a"
hear = "cmd+h"
```

### Key Configuration Options:

- Each stage (`embedding`, `autocomplete`, `chat`, `hearing`) can use a different model from different providers.
- Supported providers: [anthropic](https://www.anthropic.com/), [google](https://deepmind.google/technologies/gemini/), [openai](https://openai.com/), [groq](https://groq.com/), [ollama](https://ollama.com/), [together](https://together.ai/), [elevenlabs](https://elevenlabs.io/)
- It is strongly recommended to use an **instruction-tuned** model (like `mistral-instruct` or `gpt-3.5-turbo`) for better sentence completions.
- Avoid using "reasoning" or "chat" oriented models for autocomplete tasks.

- **Skip App**: skip specified apps (VSCode, Zed, etc.).
  - Case-insensitive — "code", "Code", or "CoDe" will all match.
- **Skip Patterns**: filters to avoid capturing sensitive content.
- **Security Skip**: extra layer to automatically ignore known private windows.
- **Autocomplete discard behavior**:
  - `full`: one backspace discards the entire suggestion.
  - `word`: each backspace deletes one word at a time.
- **Shortcuts**: customizable hotkeys (macOS only for now).
  - Case-insensitive — "cmd" or "Cmd" ("Space" or "space") will all match.

## Roadmap

- [x] Local-first memory engine
- [x] Autocomplete from memory
- [x] Terminal-based memory chat
- [x] Configurable models/providers
- [x] Sensitive data handling
- [x] Cloud LLM support (OpenAI, Claude, etc.)
- [ ] Multilingual OCR
- [x] Automatic private window skipping
- [ ] **Audio memory** (calls, meetings)
- [ ] Local web chat UI
- [ ] Raycast chat extension

## Additional Resources

- **Rust**: [https://www.rust-lang.org](https://www.rust-lang.org)
- **Ollama**: [https://ollama.com](https://ollama.com)
- **Project Vision**: [Guillermo Rauch's post](https://x.com/rauchg/status/1903528336241861113)
