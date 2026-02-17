# notion-cli
I like using Vim and Markdown. However, the native Notion web client does not support Vim bindings. This project implements a Notion cli implemented in Rust to support Markdown to Notion conversion in the terminal.

## Project structure
We use the [binary + library] crate pattern. The Notion client is a general library crate for use anywhere. The program itself handles parsing and conversion, and uses the Notion client library itself.

## Features
*wip*
