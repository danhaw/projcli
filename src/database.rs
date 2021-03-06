use std::collections::HashMap;
use postgres::Row;
use postgres::{Client, Error, NoTls};
extern crate regex;

pub struct PgDatabase {
    client: Client,
}

impl PgDatabase {
    pub fn new(
        username: &str,
        password: &str,
        host: &str,
        db_name: &str,
    ) -> Result<PgDatabase, Error> {
        let connection_string = &format!(
            "postgresql://{}:{}@{}/{}",
            username, password, host, db_name
        ); //TODO: add a unit test for this

        let client = Client::connect(connection_string, NoTls)?;
        // let client = match Client::connect(connection_string, NoTls) {
        //     Err(e) => panic!(e), //TODO: change this
        //     Ok(c) => c,
        // };

        Ok(PgDatabase { client })
    }
    pub fn create_tables(&mut self, table_create_statments: Vec<&str>) -> Result<(), Error> {
        for table_create_statment in table_create_statments {
            self.client.batch_execute(&table_create_statment)?;
        }
        Ok(())
    }

    //TODO: add Create Database method

    pub fn get_table_data(&mut self, table_name: &str) -> Result<Vec<Row>, Error> {
        let result = match self.client.query("SELECT * FROM $1", &[&(table_name)]) {
            Err(e) => return Err(e), //TODO: change this
            Ok(r) => r,
        };
        Ok(result)
    }

    pub fn add_value(&mut self, fields: HashMap<&str, &str>,  table_name: &str) -> Result<(), Error> {
        let mut insert_statment = format!("INSERT INTO {} (", table_name);
       
        for field_name in fields.keys() {
            if fields.keys().last() != Some(field_name) {
                insert_statment.push_str(&format!("{}, ", field_name));
            } else {
                insert_statment.push_str(field_name);
            }  
        }
        
        insert_statment.push_str(") VALUES(");

        for field_value in fields.values() {
            if fields.values().last() != Some(field_value) {
                insert_statment.push_str(&format!("'{}', ", field_value));
            } else {
                insert_statment.push_str(&format!("'{}'", field_value));
            }  
        }

        insert_statment.push_str(")");
        dbg!(&insert_statment);
        self.client.batch_execute(&insert_statment)?;
        Ok(())
    } 
}

//add it to the project_db.rs
pub fn initialaize_db(db: &mut PgDatabase) -> Result<(), Error> {
 
    let create_tables_statements = vec![
        "
                    CREATE TABLE IF NOT EXISTS projects (
                        id              SERIAL PRIMARY KEY,
                        title           VARCHAR(255) NOT NULL,
                        description     VARCHAR(1200),
                        start_date      TIMESTAMP,
                        end_date        TIMESTAMP
                    )
                    ",
        "
                        CREATE TABLE IF NOT EXISTS notes (
                            id              SERIAL PRIMARY KEY,
                            title           VARCHAR(255) NOT NULL,
                            body            VARCHAR(1200),
                            created_at      TIMESTAMP NOT NULL,
                            updated_at      TIMESTAMP NOT NULL,
                            project_id      INTEGER NOT NULL REFERENCES projects
                        )
                    ",
        "
                    CREATE TABLE IF NOT EXISTS todo_lists (
                        id              SERIAL PRIMARY KEY,
                        title           VARCHAR(255) NOT NULL,
                        created_at      TIMESTAMP NOT NULL,
                        updated_at      TIMESTAMP NOT NULL,
                        project_id      INTEGER NOT NULL REFERENCES projects
                    )
                    ",
        "
                        CREATE TABLE IF NOT EXISTS todo_items (
                            id              SERIAL PRIMARY KEY,
                            title           VARCHAR(255) NOT NULL,
                            is_completed    BOOLEAN,
                            created_at      TIMESTAMP NOT NULL,
                            completed_at    TIMESTAMP NOT NULL,
                            todo_list_id    INTEGER NOT NULL REFERENCES todo_lists
                        )
                        ",
        "
                        CREATE TABLE IF NOT EXISTS tags(
                            id              SERIAL PRIMARY KEY,
                            title           VARCHAR(255) NOT NULL,
                            description     VARCHAR(1200),
                            color           VARCHAR(50),
                            project_id      INTEGER NOT NULL REFERENCES projects
                        )
                    ",
    ];


    match db.create_tables(create_tables_statements) {
        Err(e) => Err(e),
        Ok(()) => {
            println!("Database tables created successfully"); //TODO: make a logger
            Ok(())
        }
    }
    
}
