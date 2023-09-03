# CliNotes
## About

CliNotes is a command-line interface (CLI) tool designed to assist developers in quickly inputting and querying information. Whether it's logging development progress, noting down learning points, or storing code snippets, CliNotes has got you covered.
Vision

### There will be three features for this application
- Dev logs
  - An append only type of log which is simple to create
  - `cliNotes devlog "today I started Rust project"(Log entry) "rust,sql,schema"(optional tags)`
  - once added they can be viewed in the app or yanked to be used somewhere else
  - One example use case can be suppose you have a journal but that is a sophisticated app that does not play nice with terminal
  - You can work on your terminal all day and keep logging everything
  - Then after the day is done simply yank it out and paste it wherever you'd like
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

```bash
cliNotes codesnip '(?:[a-z0-9!#$%&'"'"'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'"'"'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+\])' 'regex'
```
  
The primary goal of CliNotes is to create a tool that respects the UNIX philosophy—doing one thing and doing it well. It's designed to interoperate seamlessly with other programs, potentially making it a vital part of any developer's toolkit.
Design Principles

    Command-line Interface (CLI): With a focus on clear, descriptive commands, users can expect straightforward interactions. The tool utilizes CLI parsing libraries to manage command-line arguments and provide built-in help messages.

    UNIX Philosophy Adherence: CliNotes respects the time-tested UNIX philosophy. It's designed to function effectively as a standalone tool while also offering the flexibility to integrate with other tools via text streams.

    Interactive Mode: Beyond simple CLI interactions, CliNotes offers an interactive mode. This mode, resembling a Text User Interface (TUI), allows users to scroll through entries, navigate menus, and much more.

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

Interface Preview

Upon starting, users are welcomed with:
```


,-----. ,--. ,--. ,--.  ,--.           ,--.                   
'  .--./ |  | `--' |  ,'.|  |  ,---.  ,-'  '-.  ,---.   ,---.  
|  |     |  | ,--. |  |' '  | | .-. | '-.  .-' | .-. : (  .-'  
'  '--'\ |  | |  | |  | `   | ' '-' '   |  |   \   --. .-'  `)
`-----' `--' `--' `--'  `--'  `---'    `--'    `----' `----'



```scss


██████ ██      ██ ███    ██  ██████  ████████ ███████ ███████
██      ██      ██ ████   ██ ██    ██    ██    ██      ██
██      ██      ██ ██ ██  ██ ██    ██    ██    █████   ███████
██      ██      ██ ██  ██ ██ ██    ██    ██    ██           ██
██████ ███████ ██ ██   ████  ██████     ██    ███████ ███████


Welcome to CliNotes!

[1] View Dev Logs (Latest 3 entries)
[2] View Learning Notes (Latest 3 entries)
[3] View Code Snippets (Last 5 entries)
[4] Add new Code Snippet
[5] Exit

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