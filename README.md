# CliNotes - AI-Powered Journaling App
## About

CliNotes is an intelligent command-line interface (CLI) tool that transforms your development journaling experience with AI-powered insights. Whether you're logging development progress, reflecting on learning experiences, or storing code snippets, CliNotes provides smart analysis to help you understand your thoughts and feelings over time.

## Vision

### There are three main features for this application
- **AI Journal** 🤖
  - An intelligent journaling system with automatic sentiment analysis and tag generation
  - `cliNotes journal add "Today I started a Rust project and I'm excited about learning systems programming!"`
  - AI automatically analyzes sentiment (positive/negative/neutral) and generates relevant tags
  - Smart summarization: `cliNotes journal summarize --period week`
  - Intelligent insights: `cliNotes journal insights "How have I felt about my coding projects this month?"`
  - Perfect for developers who want to track their emotional journey and learning progress
  - Export capabilities to integrate with other journaling apps
- Learning Notes
  - Learning Notes aims to be a vimWiki type place but this is a long term vision for it
  - For now I am only aiming for it to be a collection of md files stored with this app being the only common connection between them
  - So basically you can have MD files scattered throughout your PC and through this app you can "collect" it together
- Code Snippets
  - Throughout the day we visit Stack Overflow or ChatGPT many times
  - We use the code there once and then either commit to git or just use the command once and then forget all about it 
  - Until we have to use Google again and research the same query
  - Sometimes the results are not the same and so such a useful snippet gets lost
  - Remember that email regex you saw on SO ? yeah, where is it now? I bet its no where
  - With cliNotes you can simply do this 
  
## AI Journal Features 🧠

The AI Journal is the flagship feature of CliNotes, providing intelligent analysis of your development journey:

### Automatic Sentiment Analysis
- **Positive Detection**: Identifies entries expressing happiness, excitement, satisfaction, and achievement
- **Negative Detection**: Recognizes frustration, sadness, challenges, and setbacks  
- **Neutral Classification**: Categorizes factual or balanced entries

### Smart Tag Generation
- **Technology Tags**: Automatically detects programming languages, frameworks, and tools mentioned
- **Activity Tags**: Identifies learning, debugging, project work, and other development activities
- **Context Tags**: Recognizes work-related, personal projects, and learning contexts

### Intelligent Summarization
- **Weekly/Monthly/Yearly Summaries**: Get AI-generated insights about your emotional patterns and focus areas
- **Sentiment Trends**: Track how your feelings about coding and projects evolve over time
- **Topic Analysis**: Understand what subjects you've been most engaged with

### Keyword-based Insights
- **Query Your Journal**: Search your journal entries using keywords.
- **Pattern Recognition**: AI identifies recurring themes and emotional patterns in your entries based on keywords.
- **Contextual Search**: Find entries by keywords, sentiment, or topics.

## Design Principles

**Command-line Interface (CLI)**: With a focus on clear, descriptive commands, users can expect straightforward interactions. The tool utilizes CLI parsing libraries to manage command-line arguments and provide built-in help messages.

**UNIX Philosophy Adherence**: CliNotes respects the time-tested UNIX philosophy. It's designed to function effectively as a standalone tool while also offering the flexibility to integrate with other tools via text streams.

**AI-Enhanced Experience**: Beyond simple CLI interactions, CliNotes offers intelligent analysis that helps you understand your development journey and emotional patterns over time.

## Getting Started
Building the Project

With the provided Makefile, building and running the project is a breeze
    Build the project:
```bash

make build

Initialize the SQLite database:

make init-db

Start the application:

make start

## Interface Preview

Upon starting, users are welcomed with:

```
██████ ██      ██ ███    ██  ██████  ████████ ███████ ███████
██      ██      ██ ████   ██ ██    ██    ██    ██      ██
██      ██      ██ ██ ██  ██ ██    ██    ██    █████   ███████
██      ██      ██ ██  ██ ██ ██    ██    ██    ██           ██
██████ ███████ ██ ██   ████  ██████     ██    ███████ ███████

Welcome to CliNotes - AI-Powered Journaling!

🤖 AI Journal Features:
[1] Add Journal Entry (with AI sentiment analysis & auto-tagging)
[2] AI Summary (weekly/monthly insights)
[3] AI Insights (ask questions about your entries)
[4] View Learning Notes (Latest 3 entries)
[5] View Code Snippets (Last 5 entries)
[6] Add new Code Snippet
[7] Exit

💡 Try: 'cargo run -- journal add "Today I learned Rust!"'
💡 Try: 'cargo run -- journal summarize --period week'
💡 Try: 'cargo run -- journal insights "Rust"'
```

## Command Examples

### Adding Journal Entries
```bash
# Basic journal entry
cargo run -- journal add "Today I started learning Rust and I'm excited about systems programming!"

# Journal entry with custom tags
cargo run -- journal add "Fixed a challenging bug in the authentication system" --tags "debugging,auth,backend"

# The AI will automatically:
# - Analyze sentiment (positive/negative/neutral)
# - Generate relevant tags (rust, learning, systems, etc.)
# - Store everything for future analysis
```

### Getting AI Summaries
```bash
# Weekly summary
cargo run -- journal summarize --period week

# Monthly summary  
cargo run -- journal summarize --period month

# Yearly summary
cargo run -- journal summarize --period year
```

### AI Insights and Search
```bash
# Search your journal by keywords
cargo run -- journal insights "Rust"
cargo run -- journal insights "debugging"
cargo run -- journal insights "learning new technologies"
```

Recommendations for Use

    Focus on User Experience: UX remains paramount. Expect clear prompts, meaningful error messages, and logical workflows.

    Consistent Syntax and Conventions: The tool follows a consistent syntax. If a flag like --devlog is being used, it'll be consistent across all commands.

    Robust Documentation: A built-in --help command explains all available options, enhancing the user experience.

    Feedback and Confirmation: After every interaction, CliNotes provides feedback, ensuring the user is always informed.
```
For a deeper dive into designing CLI apps, consider exploring:

[Command Line Interface Guidelines](https://clig.dev/) 

[12 Factor CLI Apps](https://medium.com/@jdxcode/12-factor-cli-apps-dd3c227a0e46)

Feedback

CliNotes is an evolving tool. 

Feedback from the community is invaluable for its growth. Users are encouraged to provide feedback and suggestions for improvements.