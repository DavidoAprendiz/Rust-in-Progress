<h1 align="center">CMD-Manager</h1>

<div align="center">
<img alt="Rust CI" src="https://github.com/DavidoAprendiz/Rust-in-Progress/actions/workflows/rust.yml/badge.svg">

<img alt="CMD-Manager 0.3.2" src="https://img.shields.io/badge/cmd_manager-0.3.2-000000?style=for-the-badge&color=blue">

<img alt="Made with Rust and SQLite" src="https://skillicons.dev/icons?i=rust,sqlite">
<img alt="ollama" height="50px" src="https://github.com/ollama/ollama/assets/3325447/0d0b44e2-8f4a-4e99-9b52-a5c1c741c8f7">

<a>Made with Rust, SQLite and Ollama</a>

<div align="center">
• <a href="#description">Description</a> •
  <a href="#pre-requirements">Pre-requirements</a> •
  <a href="#install">Install</a> •
  <a href="#clone">Clone</a> •
  <a href="#roadmap">Roadmap</a> •
  <a href="#contribution">Contribution</a> •
  <a href="#license">License</a> •
</div>

---

<img alt="CMD-Manager" src="https://github.com/DavidoAprendiz/CMD-Manager/assets/21132833/07356fdb-9ce9-4905-b02a-f7db84287af6">
</div>

## ⚡Description

A **multi-applications** command-line power-by AI and is focused on privacy (because privacy matters!)

From the simple to-do/note taking app to your own personal assistant database (with security logs!)

Here, you'll find the following tools:

### **To-do Manager**

- You can add, view and delete your to-dos/notes in an offline environment.
- All your to-dos/notes are saved in the folder '/Project/Todo'

### **File Manager**

- Search Mode - Search any character/word/phrase in text documents locally (similar to fzf tool).
- Compare Mode - Compare any document and see all the differences using Myers algo (similar to Git/Github diff tool).

### **Web Manager**

- Download Mode - Download simple data from web to your local drive.
- Get API Mode - Get the current price of Ergo and Cardano from Coingecko API.
- All your downloads are saved in the folder '/Project/Web'

### **Brain Manager**

- Talk with **your own** AI personal assistant and save all conversation histories in **your own** database **and/or** to markdown files. You can use them in Webpages, Github, ...
- Search for a keyword in all your conversation (questions and answers)
- Manage all your history and upkeep your database by deleting the unwanted conversations
- All your actions are audited and sent to the 'Security' database
- All your conversation are saved in the folder '/Project/Database'

## 🚀How to use

### Pre-requirements

Rust

- To run the program (without Brain module):
  - Install the latest [Rust](https://www.rust-lang.org/learn/get-started)

Ollama

- To run Brain module:
  - Install Llama3 in [Ollama](https://ollama.com/)

### Install

- You can simple run the latest standalone version:
  - [Releases](https://github.com/DavidoAprendiz/CMD-Manager/releases)

### Clone

- Open a terminal and clone the repository:
  - `git clone https://github.com/DavidoAprendiz/CMD-Manager.git`
- Enter the directory and run Cargo:
  - `cargo run`

### Manage Database via Graphical Interface (GUI)

- To view all your database information:
  - Download and install [SQLite Browser](https://sqlitebrowser.org/)
  - Open database: '/Project/Database/database.db'

## ☑️Roadmap

- [X] **Todo Manager**
  - [X] Add new tasks.
  - [X] View your saved tasks.
  - [X] Delete your saved tasks.
- [X] **File Manager**
  - [X] Search for a "keyword" in any file.
  - [X] Compare any two files and see all differences between them.
- [X] **Web Manager**
  - [X] Download any webpage from the web.
  - [X] Get data from Coingecko APIs.
- [X] Improve TUI (added colors)
- [X] Add SQLite database.
- [X] Basic unit tests (for best practices).
- [X] **Brain Manager**
  - [X] Brainstorm with your personal AI.
  - [X] You can save the answers directly in a Markdown file.
  - [X] Search 'keywords' in 'Brain' database.
  - [X] View 'Brain' history database.
  - [X] Manage 'Brain' history database.
  - [X] View 'Security' history database.
  - [X] Security Audit. Everything is logged in Security database.

## 💻Contributing

Contributions of all types are more than welcome so please feel free!

## 📃License

- [MIT](https://github.com/DavidoAprendiz/CMD-Manager/LICENSE)
