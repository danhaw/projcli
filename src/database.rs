use postgres::Row;
use postgres::{Client, NoTls, Error};
extern crate regex;



pub struct PgDatabase {
    client: Client,
} 



//macro goal:
// table_name1 {
//     field1 : type,
//     field2 : type...
// },  
// table_name2 {
//     field1 : type,
//     field2 : type...
// }...

macro_rules!  create_tables {
    ($($table_name:ident => {
        $($field:ident:$value_type:pat,)* 
    },)+) => ({
        let mut create_statments: Vec<&str> = Vec::new();
        $(
        let mut fields: std::collections::HashMap<String, String> = std::collections::HashMap::new();

        $(
            fields.insert(stringify!($field).to_string(), stringify!($value_type).to_string());
            dbg!(stringify!($value_type));
        )*
        
    
        let mut create_statement = format!("CREATE TABLE IF NOT EXISTS {} (", stringify!($table_name));
        let valid_fields:Vec<String> = fields.values().into_iter().map(|v_type|  {
            let input_re = regex::Regex::new(
                r#"(?x)
                (integer) |
                (datetime) |
                (string)\s* \((.*)\) |
                (string)         
                "#
            ).unwrap(); //(string)\s*(\d+)
        
            // Execute the Regex
            let captures = input_re.captures(v_type).map(|captures| {
                captures
                    .iter() // All the captured groups
                    .skip(1) // Skipping the complete match
                    .flat_map(|c| c) // Ignoring all empty optional matches
                    .map(|c| c.as_str()) // Grab the original strings
                    .collect::<Vec<_>>() // Create a vector
            });
            match captures.as_ref().map(|c| c.as_slice()) {
                Some(["integer"]) => return "INT".to_string(),
                Some(["datetime"]) => return "TIMESTAMP".to_string(),
                Some(["string", size]) => {
                    let s:u32= size.parse().expect("Can't parse the size of the varchar as a number");
                    let result = format!("VARCHAR({})", s).to_string();
                    return result;
                },
                Some(["string"]) => return "VARCHAR".to_string(),
                _ => return "".to_string()
            };
            
            
        }).collect::<Vec<_>>();
        for (field, value_type) in fields.keys().zip(valid_fields)  {
            if fields.keys().last().unwrap() != field {
                create_statement.push_str(&format!("{} {}, ", field, value_type));  
            } else {
                create_statement.push_str(&format!("{} {}", field, value_type));  
            }
        }
        create_statement.push_str(")");
        create_statments.push(&create_statement);
    )+
        dbg!(create_statments);
    });
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



