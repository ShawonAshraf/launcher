use serde::Serialize;
use sqlite;
use sqlite::State;

/// A struct that represents an executable that can be run.
/// # Fields
/// * `id` - An optional integer that represents the ID of the executable.
/// * `name` - A string that holds the name of the executable.
/// * `path` - A string that holds the path to the executable.
#[derive(Serialize)]
pub struct Executable {
    pub id: Option<String>,
    pub name: String,
    pub path: String,
}

/// Represents an integration with a SQLite3 database.
///
/// This struct encapsulates the connection to a SQLite3 database, allowing
/// operations related to interacting with the database.
pub struct Integration {
    db_conn: sqlite::Connection,
}

impl Integration {
    /// Creates a new instance of the `Integration` struct by opening a SQLite database connection.
    ///
    /// # Arguments
    ///
    /// * `db_name` - A string slice that holds the name or path to the SQLite database file.
    ///
    /// # Returns
    ///
    /// A `Result` containing either an `Integration` instance with a valid database connection or a static string error message if the connection fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::database::Integration;
    ///
    /// let integration = Integration::new("my_database.db".to_string());
    /// match integration {
    ///     Ok(integration) => println!("Database connection established"),
    ///     Err(e) => println!("Failed to establish database connection: {}", e),
    /// }
    /// ```
    pub fn new(db_name: String) -> Result<Integration, &'static str> {
        let connection = sqlite::Connection::open(db_name).expect("Connection should be established");
        Ok(Integration { db_conn: connection })
    }

    /// Creates a table named `executables` in the SQLite database if it does not already exist.
    ///
    /// The table has three columns:
    /// - `id`: An integer that serves as the primary key.
    /// - `name`: A text field to store the name of the exe.
    /// - `path`: A text field to store the path to the executable file of the exe.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::database::Integration;
    ///
    /// let integration = Integration::new("my_database.db".to_string()).unwrap();
    /// integration.create_table();
    /// println!("Table created successfully");
    /// ```
    pub fn create_table(&self) {
        let init_query = "create table if not exists executables (id integer primary key, name text, path text)";
        self.db_conn.execute(init_query).unwrap();
    }


    /// Inserts a new exe into the `exe` table.
    ///
    /// # Arguments
    ///
    /// * `exe` - A `Executable` struct containing the name and executable path of the exe to be inserted.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::database::Integration;
    /// use crate::database::Executable;
    ///
    /// let integration = Integration::new("my_database.db".to_string()).unwrap();
    /// let new_exe = Executable { name: "MyExe".to_string(), path: "/path/to/exe.exe".to_string() };
    /// integration.add_exe(new_exe);
    /// println!("Executable added successfully");
    /// ```
    pub fn add_exe(&self, exe: Executable) {
        let query = format!("insert into executables(name, path) values('{}', '{}')",
                            &exe.name, &exe.path);
        self.db_conn.execute(query).unwrap();
    }


    /// Deletes a exe from the `exe` table by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - A string slice that holds the ID of the exe to be deleted.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::database::sqlite3::Integration;
    ///
    /// let integration = Integration::new("my_database.db".to_string()).unwrap();
    /// integration.delete_exe("1".to_string());
    /// println!("Executable deleted successfully");
    /// ```
    pub fn delete_exe(&self, id: String) {
        let query = format!("delete from executables where id='{}'", id);
        self.db_conn.execute(query).unwrap();
    }


    /// Retrieves all executables from the `exe` table.
    ///
    /// # Returns
    ///
    /// A vector of `exe` structs representing all the executables in the database.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::database::sqlite3::Integration;
    /// use crate::database::models::exe;
    ///
    /// let integration = Integration::new("my_database.db".to_string()).unwrap();
    /// let executables = integration.list_all();
    /// for exe in executables {
    ///     println!("ID: {}, Name: {}, Exe Path: {}", exe.id, exe.name, exe.path);
    /// }
    /// ```
    pub fn list_all(&self) -> Vec<Executable> {
        let mut executables: Vec<Executable> = Vec::new();
        let query = "select * from executables";
        let mut statement = self.db_conn.prepare(query).unwrap();

        while let Ok(State::Row) = statement.next() {
            let id = Some(statement.read::<String, _>("id").unwrap());
            let name = statement.read::<String, _>("name").unwrap();
            let path = statement.read::<String, _>("path").unwrap();

            executables.push(Executable { id: Option::from(id), name, path });
        }

        executables
    }
}