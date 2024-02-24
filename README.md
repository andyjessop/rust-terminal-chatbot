# Terminal-based Chatbot with Rust and OpenAI

This project is a simple terminal-based chatbot built with Rust and leveraging the OpenAI API. It allows users to interact with OpenAI's powerful AI models directly from the command line. It was built to accompany the tutorial [How to Build a Terminal-based Chatbot with Rust and OpenAI](https://www.section.io/engineering-education/terminal-based-chatbot-rust-openai/).

## Features

- Terminal-based user interaction.
- Integration with OpenAI API for generating AI responses.
- Easy setup with minimal configuration.

## Prerequisites

Before you begin, ensure you have met the following requirements:

- Rust programming language [installation](https://www.rust-lang.org/tools/install).
- An active OpenAI API key. You can obtain one by signing up at [OpenAI](https://openai.com/).

## Getting Started

To get a local copy up and running follow these simple steps:

1. Clone the repository:
   ```bash
   git clone https://github.com/andyjessop/rust-terminal-chatbot ask
   cd ask

Create a .env file in the root of your project and add your OpenAI API key:

```dotenv
OPENAI_API_KEY=your_api_key_here
```

Install the required dependencies:

```bash
cargo build
```

Run the application:

```bash
cargo run
```

## Usage

After running the application, you will be prompted to enter your questions. Type your questions into the terminal, and press Enter. The AI response will be displayed shortly after.

To end the session, type exit and press Enter.
