
use postgres::Row;
use postgres::{Client, NoTls, Error};

pub struct PgDatabase {
    client: Client,
} 

impl PgDatabase {
    pub fn new(username: &str, password: &str, host: &str, db_name: &str) -> PgDatabase {
        let connection_string  = &format!("postgresql://{}:{}@{}/{}", username, password, host, db_name); //TODO: add a unit test for this
        let mut client = match Client::connect(connection_string, NoTls) {
            Err(e) => panic!(e), //TODO: change this
            Ok(c) => c
        };
        PgDatabase {
            client: client
        }
    }

    //TODO: add Create Database method
    
    pub fn create_tables(&mut self) -> Result<(), Error> {
        //let mut client = Client::connect("postgresql://postgres:testtest@localhost/promandb", NoTls)?;
        self.client.batch_execute("
        CREATE TABLE IF NOT EXISTS projects (
            id              SERIAL PRIMARY KEY,
            title           VARCHAR(255) NOT NULL,
            description     VARCHAR(1200),
            start_date      TIMESTAMP,
            end_date        TIMESTAMP,
            created_at      TIMESTAMP NOT NULL,
            updated_at      TIMESTAMP NOT NULL
        )
        ")?; 
        self.client.batch_execute("
            CREATE TABLE IF NOT EXISTS notes (
                id              SERIAL PRIMARY KEY,
                title           VARCHAR(255) NOT NULL,
                body            VARCHAR(1200),
                created_at      TIMESTAMP NOT NULL,
                updated_at      TIMESTAMP NOT NULL,
                project_id      INTEGER NOT NULL REFERENCES projects
            )
        ")?;

        self.client.batch_execute("
        CREATE TABLE IF NOT EXISTS todo_lists (
            id              SERIAL PRIMARY KEY,
            title           VARCHAR(255) NOT NULL,
            created_at      TIMESTAMP NOT NULL,
            updated_at      TIMESTAMP NOT NULL,
            project_id      INTEGER NOT NULL REFERENCES projects
        )
        ")?;

        self.client.batch_execute("
            CREATE TABLE IF NOT EXISTS todo_items (
                id              SERIAL PRIMARY KEY,
                title           VARCHAR(255) NOT NULL,
                is_completed    BOOLEAN,
                created_at      TIMESTAMP NOT NULL,
                completed_at    TIMESTAMP NOT NULL,
                todo_list_id    INTEGER NOT NULL REFERENCES todo_lists
            )
            ")?;

    
        self.client.batch_execute("
            CREATE TABLE IF NOT EXISTS tags(
                id              SERIAL PRIMARY KEY,
                title           VARCHAR(255) NOT NULL,
                description     VARCHAR(1200),
                color           VARCHAR(50),
                project_id      INTEGER NOT NULL REFERENCES projects
            )
        ")?;
        

        Ok(())

    }

    pub fn get_table_data(&mut self, table_name: &str) -> Result<Vec<Row>, Error> {
        let result  = match self.client.query("SELECT * FROM $1", &[&(table_name)]) {
            Err(e) => panic!(e), //TODO: change this
            Ok(r) => r
        };
        Ok(result)
    }

  
} 



