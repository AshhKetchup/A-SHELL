# A-SHELL

A-SHELL is a custom-built POSIX-compliant shell created with the help of CodeCrafters. This project is part of the ["Build Your Own Shell" Challenge](https://app.codecrafters.io/courses/shell/overview), and it aims to provide a robust foundation for a shell capable of running external programs as well as handling built-in commands.

[![progress-banner](https://backend.codecrafters.io/progress/shell/7ac389f6-9fc0-434f-9bed-9568f944a0df)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

## Features

- **REPL (Read-Eval-Print Loop):** An interactive shell interface to enter commands.
- **Basic Commands:**  
  - `echo` (including support for quoting)
  - `exit`
  - `type`
  - `cd`
  - **Error Handling:** Gracefully handles errors with informative messages.
  - **Redirection:** Supports redirection of both stdout and stderr.
- **Tab Completion:**  
  - Currently a work in progress – the goal is to implement tab completion using `rustyline`, similar to how readline works in C.

## Getting Started

1. **Prerequisites:**
   - Ensure you have Rust installed (tested with `cargo 1.82`).
   
2. **Running the Shell:**
   - Use the provided shell script to compile and run the project:
     ```sh
     ./your_program.sh
     ```
   - The first run may be slow as the project compiles, but subsequent runs will be faster.


## Next Steps

This shell is a work in progress and further stages will include additional features and improvements. Stay tuned for:
- Enhanced tab completion
- Extended built-in command support
- More robust error handling and feature enhancements

---
