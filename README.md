# Task Manager

A full-stack task management application built with Rust and Dioxus.

## About The Project

This project is a simple yet powerful to-do application that can be deployed on both web and desktop platforms from a single Rust codebase. It uses `rusqlite` on the backend for persistent storage.

### Current Functionality

The application currently supports the following features:
- **Add Tasks**: Create new tasks with a description and an optional due date and time.
- **View Tasks**: See a list of all your current tasks.
- **Mark as Done/Undone**: Toggle the completion status of each task with a checkbox.
- **Delete Tasks**: Remove tasks you no longer need.
- **Persistent Storage**: Tasks are saved in a local SQLite database file (`todos.db`).

## Future Goals

The long-term vision is to transform this application into an intelligent task sorter that helps you manage your time effectively. The planned features are:

- **Advanced Task Properties**: Input tasks with details like importance and estimated time required.
- **Smart Sorting**: An engine (potentially using `burn` for ML) to automatically schedule and prioritize your tasks.
- **Task Splitting**: Break down large tasks into smaller, manageable portions.
- **Schedule Integration**: Respect the user's existing schedule.
- **Calendar UI**: A visual calendar interface to view and manage scheduled tasks, with popups for daily details.

## Getting Started

Follow these instructions to get a copy of the project up and running on your local machine.

### Prerequisites

- **Rust**: Ensure you have the latest stable version of Rust installed. You can get it from rustup.rs.
- **Dioxus CLI**: You'll need the Dioxus command-line tool to build and serve the application.
  ```sh
  cargo binstall dioxus-cli
  ```

### Running the Application

1.  Clone the repository:
    ```sh
    git clone https://github.com/SeerBlazeJ/task-manager
    cd task_sorter
    ```

2.  **Database Setup**:
    The project uses a SQLite database file named `todos.db` to store tasks. This file is included in `.gitignore` and will not be checked into version control.

    The application will automatically create `todos.db` and the necessary `todos` table in the project's root directory the first time you run it.

3.  **Serve the App**:
    You can run the application on different platforms using the Dioxus CLI.

    **For Web:**
    ```bash
    dx serve --platform web
    ```
    This will start a local development server and open the app in your default web browser.

    **For Desktop:**
    ```bash
    dx serve --platform desktop
    ```
    This will build and launch a native desktop window for your operating system.
