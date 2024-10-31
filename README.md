[![Rust CI/CD Pipeline](https://github.com/nogibjj/individual_project_2_yl/actions/workflows/ci.yml/badge.svg)](https://github.com/nogibjj/individual_project_2_yl/actions/workflows/ci.yml)

# individual_project_2_yl
# Project #2: Rust CLI Binary with SQLite

[Demo Video](https://www.youtube.com/watch?v=3bTg9vB4alA&ab_channel=stargisldf)

## Requirements
 - Rust source code: The code should comprehensively understand Rust's syntax and unique features.
    - Implementation of Rust's unique features
    - Effective error handling in Rust
    - Proper usage of Rust syntax
 - Use of LLM: In your README, explain how you utilized an LLM in your coding process.
    - Dependencies and how to install them
    - Explanation of the project
    - How to run the program
 - SQLite Database: Include a SQLite database and demonstrate CRUD (Create, Read, Update, Delete) operations.
    - Implementation of Rust's unique features
    - Effective error handling in Rust
    - Proper usage of Rust syntax
 - Optimized Rust Binary: Include a process that generates an optimized Rust binary as a Gitlab Actions artifact that can be downloaded.
 - README.md: A file that clearly explains what the project does, its dependencies, how to run the program, and how Gitlab Copilot was used.
    - Dependencies and how to install them
    - Explanation of the project
    - How to run the program
 - Github/Gitlab Actions: A workflow file that tests, builds, and lints your Rust code.
    - Correct linting of Rust code
    - Correct building of Rust code
    - Correct testing of Rust code
 - Video Demo: A YouTube link in README.md showing a clear, concise walkthrough and demonstration of your CLI binary.
    - Quality of video and audio
    - Clarity of explanation
    - Quality demonstration of the project

## Goal
This project delivers a comprehensive Data Extraction, Transformation, Loading (ETL) tool, alongside Querying (CRUD) capabilities, all developed using Rust. The process entails initializing a new Rust project with cargo init and installing Rust dependencies through Cargo.toml using cargo build. Notably, Github Copilot was employed to aid the transition from Python to Rust. This conversion was meticulously crafted to ensure adherence to Rust's syntax, robust error handling, and full utilization of Rust's unique features.

This toolkit offers a suite of functions for ETL operations on datasets, facilitating queries on a SQLite database. It comprehensively covers CRUD (Create, Read, Update, Delete) operations, logging all queries to a Markdown file, query_log.md, to aid in the tracking and analysis of executed commands.

The operational workflow includes running a Makefile to perform tasks such as installation (make install), testing (make test), code formatting (make format) with Python Black, linting (make lint) with Ruff, and an all-inclusive task (make all). This automation streamlines the data analysis process and enhances code quality.

To cap it off, the project produces an optimized Rust binary, which is available as a GitHub Actions artifact, ready for download.

## Procedure
1. Rust Initiation using `cargo init`
    initialize a new Rust project by running `cargo init` inside the directory, it will set up a new Rust project by:
    - Creating a `Cargo.toml` file, which contains configuration data, dependencies, and other metadata about the Rust project.
    - Creating a `src` directory with a main.rs file for binary projects or `lib.rs` for libraries.
    - Generating a `.gitignore` file if the directory is not inside an existing git repository.
2. Rust Dependencies Installation using `cargo build`
    Project Metadata:
    - It provides metadata about the Rust package, such as its name, version, authors, and edition.
    ```
    [package]
    name = "individual_project_2_yl"
    version = "0.1.0"
    edition = "2021"
    ```
    - Dependencies: It lists external packages (also known as "crates") that the project depends on. This allows Cargo to automatically fetch and build these dependencies when compiling the project.
    ```
    [dependencies]
    You can find and download the uploaded artifact by going to actions and clicking on the latest workflow run.

    reqwest= { version = "^0.11", features = ["blocking"] }
    rusqlite = "^0.29"
    csv = "^1.0"
    assert_cmd = "^2.0"
    predicates = "0.9"  
    ```
The Following Steps Are Performed Using `Github Copilot` Translation From Python to Rust

3. Query creation in `lib.rs` in `src`
    The `lib.rs` file provides a set of functions that work in tandem to extract data from a URL, store it as a local CSV file, load this data into a SQLite database, and provide querying capabilities, with all executed queries being logged for future reference.

4. CLI through `main.rs` in `src`
    The main.rs file provides a command-line interface (CLI) for users to execute three main actions (ETL-Query) related to a dataset: extracting it from a URL, transforming and loading it into a SQLite database , and executing SQL queries against the database.

5. Tests `etl.test.rs` in `tests`
    The etl_tests.rs file provides unit tests for the core ETL functions of the tina_yi_sqlite crate, helping ensure the integrity and correctness of the crate's functionality.

6. Makefile:

    The Makefile provides a set of tasks to automate various aspects of developing, testing, and managing a Rust project.

    Custom tasks related to the database:

    - extract: Runs the extract action of the project.
    - transform_load: Runs the transform_load action of the project.
    - create: Executes a SQL query to insert a new record into the candy_data_DB table.
    - read: Executes a SQL query to select a record from the candy_data_DB table.
    - update: Executes a SQL query to update a specific record in the candy_data_DB table.
    - delete: Executes a SQL query to delete a specific record from the candy_data_DB table.

    ```
    # Custom tasks

    # Example: Extract data
    extract: 
        cargo run extract

    # Example: Transform and Load data
    transform_load:
        cargo run transform_load

    # Example: Create a database entry
    create:
        cargo run query "INSERT INTO candy_data_DB (competitorname, chocolate, fruity, caramel, peanutyalmondy, nougat, crispedricewafer, hard, bar, pluribus) VALUES ('Grand Rabbit', 0, 0, 0, 0, 0, 0, 0, 0, 0);"


    # Example: Read from the database
    read:
        cargo run query "SELECT * FROM candy_data_DB WHERE competitorname = 'Grand Rabbit';"

    # Example: Update a database entry
    update:
        cargo run query "UPDATE candy_data_DB SET competitorname='Grand Rabbit', chocolate=0, fruity=1, caramel=0, peanutyalmondy=0, nougat=0, crispedricewafer=0, hard=0, bar=0, pluribus=0 WHERE id=86;"

    # Example: Delete a database entry
    delete:
        cargo run query "DELETE FROM candy_data_DB WHERE id=86;"
    ```

7. Github Actions ci.yml
    The CI/CD pipeline provides a comprehensive process for building, testing, and managing this Rust project on GitHub.

8. log of successful database operations
    All the data base operations were logged in the `query_log.md` file for reference. All the logs show success.

## Make Format, Test, Lint, All Approval Image
- Format code make format
- Lint code make lint
- Test code make test

## Workflow Overview
```
- Rust Initiation and Dependencies Installation (`Cargo init`, `Cargo.toml`, `Cargo build`)

- Github Copilot Translation from Python to Rust

    - Proper usage of Rust syntax

    - Effective error handling in Rust

    - Implementation of Rust's unique features

- ETL-Query:  [E] Extract a dataset from URL, [T] Transform, [L] Load into SQLite Database and [Q] Query

    - [E] The extract function downloads data from a specified URL and saves it to a local file.

    - [T][L] The transform_load function reads a CSV dataset and inserts its records into a SQLite database after performing necessary table operations. It creates a table named AirlineSafetyDB with specific columns.

    - [Q] The query function writes and executes SQL queries on the SQLite database to analyze and retrieve insights from the data. The queries can perform CRUD (create, read, update, delete) operations. 

- Logging:  The log_query function appends SQL queries to a log file. By logging the queries into a Markdown file named `query_log.md`, it facilitates tracking and analysis of executed queries.

- GitHub Actions: A workflow file that tests, builds, and lints the Rust code.

- Optimized Rust Binary: Generates an optimized Rust binary as a GitHub Actions artifact that can be downloaded.
```

## Optimized Rust Binary
The uploaded artifact can be found and downloaded by going to actions and clicking on the latest workflow run.

## Video Demo
The tutorial video demonstrates the CLI binary by covering the following:
- open codespaces and wait for codespaces to be built
- build: cargo build for dependencies installation
- extract: make extract
- transform and load: make transform_load
- query sample: you can use make create, make read, make update, or make delete to see sample CRUD Operations
- query your own: cargo run query

The successful CRUD operations in the `query_log.md` file for reference.

The uploaded artifact can be found and downloaded by going to actions and clicking on the latest workflow run.

## Dataset Reference
The dataset is from fivethirtyeight. It contains the data behind the story The Ultimate Halloween Candy Power Ranking. `candy-data.csv` includes attributes for each candy along with its ranking. For binary variables, 1 means yes, 0 means no.

