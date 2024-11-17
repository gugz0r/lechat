# Lechat IRC Client

An IRC client built with Rust and Tauri 2.

## Why?

For fun and for the prof^W^W^W^W challenge!

### Version 0.1

The goal of version 0.1 is to create a functional and user-friendly IRC client with a clean, easily maintainable code structure.

### Future Enhancements

Future versions will introduce features like copy-paste functionality, backend storage, and more.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Launching in dev/debug mode

Clone the project using git clone.

```shell
cd lechat
pnpm install
pnpm tauri dev
```

## Files structure

In src-tauri we have the rust code.
In src we have the frontend in vanilla JS/HTML.

lechat/
├── src-tauri/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands.rs
│   │   ├── irc_client/
│   │   │   ├── mod.rs
│   │   │   ├── connection.rs
│   │   │   ├── message_handler.rs
│   │   │   └── utils.rs
│   │   └── storage/
│   │       ├── mod.rs
│   │       ├── filesystem.rs
│   │       └── database.rs
├── src/
│   ├── index.html
│   ├── css/
│   │   └── styles.css
│   └── js/
│       ├── main.js
|       ├── storage.js
│       ├── ui/
│       │   ├── mod.js
│       │   ├── components.js
│       │   └── events.js
│       └── irc/
│           ├── mod.js
│           ├── connection.js
│           └── message.js
├── package.json
└── ...

## Licence

GPLv3.
