use reqwest::blocking::Client;
use rusqlite::{params, Connection, Result};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

const LOG_FILE: &str = "query_log.md";

fn log_query(query: &str, log_file: &str) {
    if let Ok(mut file) = OpenOptions::new().append(true).create(true).open(log_file) {
        if let Err(err) = writeln!(file, "```sql\n{}\n```\n", query) {
            eprintln!("Error writing to log file: {:?}", err);
        }
    } else {
        eprintln!("Error opening log file for writing.");
    }
}

pub fn extract(url: &str, file_path: &str, directory: &str) {
    if !fs::metadata(directory).is_ok() {
        fs::create_dir_all(directory).expect("Failed to create directory");
    }

    let client = Client::new();
    let mut response = client.get(url).send().expect("Failed to send request");
    let mut file = fs::File::create(file_path).expect("Failed to create file");

    std::io::copy(&mut response, &mut file).expect("Failed to copy content");

    println!("Extraction successful!");
}

pub fn transform_load(dataset: &str) -> Result<String> {
    let conn = Connection::open("candy_data_DB.db")?;

    conn.execute("DROP TABLE IF EXISTS candy_data_DB", [])?;

    conn.execute(
        "CREATE TABLE candy_data_DB (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            competitorname TEXT,
            chocolate INTEGER,
            fruity INTEGER,
            caramel INTEGER,
            peanutyalmondy INTEGER,
            nougat INTEGER,
            crispedricewafer INTEGER,
            hard INTEGER,
            bar INTEGER,
            pluribus INTEGER
        )",
        [],
    )?;

    let mut rdr = csv::Reader::from_path(dataset).expect("Failed to read dataset");

    let mut stmt = conn.prepare(
        "INSERT INTO candy_data_DB(
            competitorname,
            chocolate,
            fruity,
            caramel,
            peanutyalmondy,
            nougat,
            crispedricewafer,
            hard,
            bar,
            pluribus
        ) 
        VALUES (?,?, ?, ?, ?, ?, ?, ?,?, ?)",
    )?;

    for result in rdr.records() {
        match result {
            Ok(record) => {
                stmt.execute(&[
                    &record[0], &record[1], &record[2], &record[3], &record[4], &record[5],
                    &record[6], &record[7], &record[8], &record[9],
                ])?;
            }
            Err(err) => {
                eprintln!("Error reading CSV record: {:?}", err);
            }
        }
    }

    Ok("candy_data_DB.db".to_string())
}

pub fn query(query: &str) -> Result<()> {
    let conn = Connection::open("candy_data_DB.db")?;
    // Read operation
    if query.trim().to_lowercase().starts_with("select") {
        let mut stmt = conn.prepare(query)?;
        let results = stmt.query_map(params![], |row| {
            Ok((
                row.get::<usize, i32>(0)?,
                row.get::<usize, String>(1)?,
                row.get::<usize, i32>(2)?,
                row.get::<usize, i32>(3)?,
                row.get::<usize, i32>(4)?,
                row.get::<usize, i32>(5)?,
                row.get::<usize, i32>(6)?,
                row.get::<usize, i32>(7)?,
                row.get::<usize, i32>(8)?,
                row.get::<usize, i32>(9)?,
                row.get::<usize, i32>(10)?,
            ))
        })?;

        for result in results {
            match result {
                Ok((
                    id,
                    competitorname,
                    chocolate,
                    fruity,
                    caramel,
                    peanutyalmondy,
                    nougat,
                    crispedricewafer,
                    hard,
                    bar,
                    pluribus,
                )) => {
                    println!(
                        "Result: id={}, competitorname={}, chocolate={}, fruity={}, caramel={}, peanutyalmondy={}, nougat={}, crispedricewafer={}, hard={}, bar={}, pluribus={}",
                        id, competitorname, chocolate, fruity, caramel, peanutyalmondy, nougat, crispedricewafer, hard, bar, pluribus
                    );
                }
                Err(e) => eprintln!("Error in row: {:?}", e),
            }
        }
    } else {
        // other CUD operations
        let _num_affected_rows = conn.execute_batch(query)?;
    }
    log_query(query, LOG_FILE);
    Ok(())
}
