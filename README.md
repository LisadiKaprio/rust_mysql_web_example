# Rust MySQL Web 💻 Example

A simple Rust application that uses SQLx to interact with a MySQL database.

More specifically, it manages a database of Stardew Valley characters (for the sake of an example). Each character has a name, a birthday, a favourite gift item and whether or not they're available for marriage.

This is a Rust backend for displaying information in a web frontend.

## Installation

To run the server side, you need to have Rust and MySQL installed on your machine. To test it using web frontend, you need Node.js and npm package manager installed on your machine.

1. [Install Rust.](https://www.rust-lang.org/tools/install)

2. [Install MySQL.](https://dev.mysql.com/doc/refman/8.0/en/installing.html)

3. [Install Node.js.](https://nodejs.org/en/download/package-manager)

3. In backend folder, copy the already existing `.env.example` file in the root folder and rename it to `.env`. Change the credentials in the first line to be the ones that you have previously set up in your MySQL environment.

4. Run the setup script to create the database:
    ```console
    cd backend
    cargo run --bin setup_db
    ```

    Alternatively, you can create a database manually and just fill in the proper values in the `.env` file.

5. You can now run the server side of the application:
    ```console
    cd backend
    cargo run --bin main
    ```

6. To start the frontend, do the following:

    ```console
    cd frontend
    npm ci
    npm run dev
    ```

