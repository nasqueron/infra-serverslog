/*  -------------------------------------------------------------
    Servers log microservice
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    Project:        Nasqueron
    Created:        2016-11-11
    License:        BSD-2-Clause
    -------------------------------------------------------------    */

use sqlite3::DatabaseConnection;
use sqlite3::SqliteResult;
use sqlite3::StatementUpdate;
use sqlite3::ToSql;
use sqlite3::access::ByFilename;

use std::env;
use std::fs;
use std::io;

/*  -------------------------------------------------------------
    Log entry
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

pub struct LogEntry {
    pub date: String,
    pub emitter: String,
    pub source: String,
    pub component: String,
    pub entry: String,
}

/*  -------------------------------------------------------------
    Data store context
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

pub struct DataStore {
    /// The path to the SQLite database filename
    /// By default, honours $STORE environment variable, or if omitted, "./log.db".
    filename: String,

    /// The connexion to the database
    connection: DatabaseConnection,
}

impl DataStore {
    /// Initializes a new instance of the DataStore object.
    pub fn new () -> Result<DataStore, String> {
        let path = ::store::get_path();
        match ::store::get_connection() {
            Ok(database_connection) => {
                let mut store = DataStore {
                    filename: path,
                    connection: database_connection
                };
                store.init();
                Ok(store)
            },
            Err(err) => Err(err.desc.to_string())
        }
    }

    /// Initializes the data store.
    pub fn init(&mut self) {
        self.connection.exec(::store::get_schema()).unwrap();
    }

    /// Destroys the data store.
    pub fn destroy (&self) -> Result<(), io::Error> {
        try!(fs::remove_file(&*self.filename));
        info!("Destroyed {}", self.filename);
        Ok(())
    }

    /// Executes a prepared statement.
    ///   - query: the SQL query, each parameter replaced by $1, $2, etc.
    ///   - parameters: the parameters to put in the query
    pub fn exec_prepared_statement (&self, query: &str, parameters: &[&ToSql]) -> Result<u64, String> {
        match self.connection.prepare(query).unwrap().update(parameters) {
            Ok(rows_updated) => Ok(rows_updated),
            Err(err) => Err(err.desc.to_string()),
        }
    }

    /// Inserts an entry to the log.
    pub fn insert (&self, entry: LogEntry) -> Result<u64, String> {
        debug!("Inserting new log entry");
        self.exec_prepared_statement(
            "INSERT INTO log (date, emitter, source, component, entry)
             VALUES ($1, $2, $3, $4, $5);",
            &[&entry.date, &entry.emitter, &entry.source, &entry.component, &entry.entry]
        )
    }
}

/*  -------------------------------------------------------------
    Helper functions — Database
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

/// Opens a connection to the database.
fn get_connection () -> SqliteResult<DatabaseConnection> {
    let path = &*get_path();
    info!("Opening database {}", path);

    DatabaseConnection::new(
        ByFilename {
            filename: path,
            flags: Default::default(),
        }
    )
}

/// Gets the SQL tables schema for the log store.
pub fn get_schema<'a> () -> &'a str {
    include_str!("../sql/schema.sql")
}

/*  -------------------------------------------------------------
    Helper functions
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

/// Gets the path to the store to use.
fn get_path() -> String {
    env::var("STORE").unwrap_or("log.db".to_string())
}

/*  -------------------------------------------------------------
    Tests
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

#[cfg(test)]
mod tests {

    use super::get_path;
    use super::get_schema;
    use super::DataStore;
    use super::LogEntry;

    use std::path::Path;

    //
    // SQL schema tests
    //

    #[test]
    fn get_schema_creates_a_table() {
        assert!(get_schema().contains("CREATE TABLE"))
    }

    #[test]
    fn get_schema_wont_fail_if_table_already_exists() {
        assert!(get_schema().contains("IF NOT EXISTS"))
    }

    //
    // SQLite file tests
    //

    /// Determines if the store exists.
    fn store_exists () -> bool {
        Path::new(&get_path()).exists()
    }

    #[test]
    fn get_store_path_returns_expected_default_value() {
        assert_eq!("log.db", get_path());
    }

    #[test]
    fn store_exists_when_initialized_but_not_after_destroy() {
        let store = DataStore::new().unwrap();
        assert_eq!(true, store_exists(), "Store doesn't exist after initialization.");

        store.destroy().unwrap();
        assert_eq!(false, store_exists(), "Store still exists after being destroyed.");
    }

    //
    // CRUD tests
    //

    #[test]
    fn insert_adds_a_row() {
        let store = DataStore::new().unwrap();
        let rows_updated = store.insert(
            LogEntry {
                date: String::from("2016-03-30T13:03:00Z"),
                emitter: String::from("Sandlayth"),
                source: String::from("#nasqueron-ops"),
                component: String::from("Dwellers"),
                entry: String::from("docker start wolfphab"),
            }
        ).unwrap();
        assert_eq!(1, rows_updated);

        store.destroy().unwrap();
    }

}
