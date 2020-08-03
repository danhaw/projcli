// use postgres::Row;
// use postgres::{Client, Error, NoTls};
// extern crate regex;

// pub struct PgDatabase {
//     client: Client,
// }

// macro_rules! create_tables {


//     //match this pattern
//     // table_name1 {
//     //     column1 : type,
//     //     column2 : type...
//     // },
//     // table_name2 {
//     //     column1 : type,
//     //     column2 : type...
//     // }...
//     ($($table_name:ident => {
//         $($column:ident:$value_type:pat, $req:literal $(,)?)*
//     }$(,)?)+) => ({
//          //this vector will hold each "create table statement" that user provided and it
//          //will be returned at the end of the macro
//         let mut create_statements: Vec<String> = Vec::new();

//         $( //notice repeating pattern for each table provided by the user

//         //can't use the Hashmap because it doesn't keep the insertation order

//         //this vector will hold the column name
//         let mut columns: Vec<String> = Vec::new();
//         //this vector will hold the type of the column
//         let mut types: Vec<(String, bool)> = Vec::new();
//         //they will be linked with a zip function later in the code


//         //this pattern will repeat depending on the number of the inner repetition of the columns
//         //and their types provided in the macro, and will assign each value(column_name, type_name) to the vectors
//         //I don't have to assert equality of their lengths because the macro pattern provided will
//         //ensure that and throw an error at compile time if they don't have an equal length
//         $(
//             columns.push(stringify!($column).to_string());

//             //types.push(stringify!($value_type).to_string());
//             if $req == "required"{
//                 types.push((stringify!($value_type).to_string(), true));
//             }
//             types.push((stringify!($value_type).to_string(), false));

//         )*

//         //constructing the sql create statement it's quite simple for now
//         //but I will try to improve it later to be able to generate more complex ones
//         let mut create_statement = format!("CREATE TABLE IF NOT EXISTS {} (", stringify!($table_name));

//         //from here the code will replace the abstracted types provided by the users to
//         //their corresponding sql valid types using Regular expressions to capture them
//         //I have two better options to deal with the wrong types either by returning None and handling it or panicing

//         //not sure for now so I'm returning an empty string and I will be changing the match statement
//         //to a separate function and making its test fail if that happened TODO:
//         //that's for now, at least until I provide an implementation for the macro where I have to deal with it
//         let valid_columns:Vec<String> = types.into_iter().map(|v_type|  {
//             let input_re = regex::Regex::new(
//                 r#"(?x)
//                 (primary) |
//                 (reference)\s* \((.*)\) |
//                 (integer) |
//                 (datetime) |
//                 (string)\s* \((.*)\) |
//                 (string)
//                 "# //(string)\s* \((\d*),\s*(\required)\)      //(string)\s* ([^,]+\(.+?\))|([^,]+) | (string)\s*[\w\s$]*(\((.*[\w\s,$]*)\))
//             ).unwrap(); //TODO: are more types TODO: add Required -> NOT NULL type

//             // Execute the Regex
//             let captures = input_re.captures(&v_type.0).map(|captures| {
//                 captures
//                     .iter() // All the captured groups
//                     .skip(1) // Skipping the complete match
//                     .flat_map(|c| c) // Ignoring all empty optional matches
//                     .map(|c| c.as_str()) // Grab the original strings
//                     .collect::<Vec<_>>() // Create a vector
//             });

//             if !v_type.1 {
//                  //change each type with corresponding sql valid types
//             match captures.as_ref().map(|c| c.as_slice()) {
//                 //returning the converted type directly to the lambda function provided for the map
//                 //to replace the current user provided type
//                 Some(["primary"]) => return "SERIAL PRIMARY KEY".to_string(),
//                 Some(["integer"]) => return "INTEGER".to_string(),
//                 Some(["datetime"]) => return "TIMESTAMP".to_string(),
//                 Some(["string", size]) => {
//                     //in case the string has a size ex: string(50) converts to VARCHAR(50)
//                     let s:u32= size.parse().expect("Can't parse the size of the varchar as a number");
//                     let result = format!("VARCHAR({})", s).to_string();
//                     return result;
//                 },
//                 // Some(["string", size, required]) => {
//                 //     //in case the string has a size ex: string(50) converts to VARCHAR(50)
//                 //     let s:u32= size.parse().expect("Can't parse the size of the varchar as a number");
//                 //     let result;
//                 //     if required == &"required" {
//                 //         result = format!("VARCHAR({}) NOT NULL", s).to_string();
//                 //     } else {
//                 //         result = format!("VARCHAR({})", s).to_string();
//                 //     }
//                 //     return result;
//                 // },
//                 Some(["string"]) => return "VARCHAR".to_string(),
//                 Some(["reference", referenced_table_name]) => {
//                     //this for the foreign key it has a reference to another table
//                     //ex: reference(table_name) converts to INTEGER REFERENCE table_name
//                     let result = format!("INTEGER REFERENCES {}", referenced_table_name).to_string();
//                     return result;
//                 },
//                 _ => return "".to_string() //TODO: change this
//             }; //end of the converting

//         } else {
//             match captures.as_ref().map(|c| c.as_slice()) {
//                 //returning the converted type directly to the lambda function provided for the map
//                 //to replace the current user provided type
//                 Some(["primary"]) => return "SERIAL PRIMARY KEY".to_string(),
//                 Some(["integer"]) => return "INTEGER NOT NULL".to_string(),
//                 Some(["datetime"]) => return "TIMESTAMP NOT NULL".to_string(),
//                 Some(["string", size]) => {
//                     //in case the string has a size ex: string(50) converts to VARCHAR(50)
//                     let s:u32= size.parse().expect("Can't parse the size of the varchar as a number");
//                     let result = format!("VARCHAR({}) NOT NULL", s).to_string();
//                     return result;
//                 },
//                 // Some(["string", size, required]) => {
//                 //     //in case the string has a size ex: string(50) converts to VARCHAR(50)
//                 //     let s:u32= size.parse().expect("Can't parse the size of the varchar as a number");
//                 //     let result;
//                 //     if required == &"required" {
//                 //         result = format!("VARCHAR({}) NOT NULL", s).to_string();
//                 //     } else {
//                 //         result = format!("VARCHAR({})", s).to_string();
//                 //     }
//                 //     return result;
//                 // },
//                 Some(["string"]) => return "VARCHAR NOT NULL".to_string(),
//                 Some(["reference", referenced_table_name]) => {
//                     //this for the foreign key it has a reference to another table
//                     //ex: reference(table_name) converts to INTEGER REFERENCE table_name
//                     let result = format!("INTEGER REFERENCES {} NOT NULL", referenced_table_name).to_string();
//                     return result;
//                 },
//                 _ => return "".to_string() //TODO: change this
//             }; //end of the converting
//         }
//         }).collect::<Vec<_>>(); //end of the allocation to the valid_types vector




//         //back to constructing the "create table statement" ,
//         //adding the columns names with their types in the create statement

//         for (column, value_type) in  columns.iter().zip(valid_columns) {
//             if columns.iter().last() != Some(column) {
//                 create_statement.push_str(&format!("{} {}, ", column, value_type));
//             } else { //if we reached the last value then remove the tailing comma
//                 create_statement.push_str(&format!("{} {}", column, value_type));
//             }
//         }
//         create_statement.push_str(")"); //finish the create statement
//         create_statements.push(create_statement); //push it to other statements for other tables
//     )+ //notice pattern repetition for each table ends here

//     //return all the valid(hopefully) sql statements created depending on the user input
//     create_statements
//     });

// }

// impl PgDatabase {
//     pub fn new(
//         username: &str,
//         password: &str,
//         host: &str,
//         db_name: &str,
//     ) -> Result<PgDatabase, Error> {
//         let connection_string = &format!(
//             "postgresql://{}:{}@{}/{}",
//             username, password, host, db_name
//         ); //TODO: add a unit test for this

//         let client = Client::connect(connection_string, NoTls)?;
//         // let client = match Client::connect(connection_string, NoTls) {
//         //     Err(e) => panic!(e), //TODO: change this
//         //     Ok(c) => c,
//         // };

//         Ok(PgDatabase { client })
//     }
//     pub fn create_tables(&mut self, table_create_statments: Vec<String>) -> Result<(), Error> {
//         for table_create_statment in table_create_statments {
//             self.client.batch_execute(&table_create_statment)?;
//         }
//         Ok(())
//     }

//     //TODO: add Create Database method

//     pub fn get_table_data(&mut self, table_name: &str) -> Result<Vec<Row>, Error> {
//         let result = match self.client.query("SELECT * FROM $1", &[&(table_name)]) {
//             Err(e) => panic!(e), //TODO: change this
//             Ok(r) => r,
//         };
//         Ok(result)
//     }
// }

// //add it to the project_db.rs
// pub fn initialaize_db() -> Result<(), Error> {
//     let mut db = PgDatabase::new("postgres", "testtest", "localhost", "promandb")?;
//     let create_tables_statements: Vec<String> = create_tables!(
//         projects => {
//             id: primary, " ",
//             title: string, "required",
//             description: string(1000), "required",
//             start_date: datetime, "",
//             end_date: datetime, ""
//         },
//         notes => {
//             id: primary, "",
//             title: string, "",
//             body: string(1500), "",
//             created_at: datetime, "",
//             updated_at: datetime, "",
//             project_id: reference(projects), "",
//         }
//     );
//     match db.create_tables(create_tables_statements) {
//         Err(e) => panic!(e),
//         Ok(()) => {
//             println!("Database tables created successfully");
//             Ok(())
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_create_tables_macro_returns_one_table_create_statements_with_basic_types() {
//         let create_tables_statements: Vec<String> = create_tables!(
//             projects => {
//                 id: primary, "",
//                 title: string, "",
//                 description: string(1000), "",
//                 start_date: datetime, "",
//                 end_date: datetime, "",

//             }
//         );
//         assert_eq!(create_tables_statements.len(), 1);
//         assert_eq!(create_tables_statements[0], "CREATE TABLE IF NOT EXISTS projects (id SERIAL PRIMARY KEY, title VARCHAR, description VARCHAR(1000), start_date TIMESTAMP, end_date TIMESTAMP)");
//     }
//     #[test]
//     fn test_create_tables_macro_returns_two_table_create_statements_with_basic_types() {
//         let create_tables_statements: Vec<String> = create_tables!(
//             projects => {
//                 id: primary, "",
//                 title: string, "",
//                 description: string(1000), "",
//                 start_date: datetime, "",
//                 end_date: datetime, "",
//             },
//             notes => {
//                 id: primary, "",
//                 title: string, "",
//                 body: string(1500), ""
//                 created_at: datetime, "",
//                 updated_at: datetime, "",
//                 project_id: reference(projects), "",
//             }
//         );
//         assert_eq!(create_tables_statements.len(), 2);
//         assert_eq!(create_tables_statements[0], "CREATE TABLE IF NOT EXISTS projects (id SERIAL PRIMARY KEY, title VARCHAR, description VARCHAR(1000), start_date TIMESTAMP, end_date TIMESTAMP)");
//         assert_eq!(create_tables_statements[1], "CREATE TABLE IF NOT EXISTS notes (id SERIAL PRIMARY KEY, title VARCHAR, body VARCHAR(1500), created_at TIMESTAMP, updated_at TIMESTAMP, project_id INTEGER REFERENCES projects)");
//     }

//     #[test]
//     fn test_create_tables_macro_returns_one_table_create_statements_with_required() {
//         let create_tables_statements: Vec<String> = create_tables!(
//                 projects => {
//                 id: primary, "",
//                 title: string, "required",
//                 test: integer, "",
//                 description: string(1000), "",
//                 start_date: datetime, "",
//                 end_date: datetime, "",

//             }
//         );
//         assert_eq!(create_tables_statements[0], "CREATE TABLE IF NOT EXISTS projects (id SERIAL PRIMARY KEY, title VARCHAR NOT NULL, test INTEGER, description VARCHAR(1000), start_date TIMESTAMP, end_date TIMESTAMP)");

//         // let create_tables_statements: Vec<String> = create_tables!(
//         //     projects => {
//         //         id: primary,
//         //         title: string(required),
//         //         description: string(1000, required),
//         //         start_date: datetime,
//         //         end_date: datetime
//         //
//         //     }
//         // );
//         // assert_eq!(create_tables_statements.len(), 1);
//         // assert_eq!(create_tables_statements[0], "CREATE TABLE IF NOT EXISTS projects (id SERIAL PRIMARY KEY, title VARCHAR NOT NULL, description VARCHAR(1000) NOT NULL, start_date TIMESTAMP, end_date TIMESTAMP)");
//     }
//     // #[test]
//     // fn test_create_tables_macro_returns_create_statements(){
//     //     let create_tables_statements = create_tables!(
//     //         projects => {
//     //             id: integer,
//     //             title: string,
//     //         },
//     //         notes => {
//     //             id: integer,
//     //             title: string,
//     //             body: string(1500),
//     //             project_id: integer,
//     //         },
//     //     );
//     //     assert_eq!(create_tables_statements[0], "CREATE TABLE IF NOT EXISTS projects (
//     //         id              SERIAL PRIMARY KEY,
//     //         title           VARCHAR(255) NOT NULL,
//     //         description     VARCHAR(1200),
//     //         start_date      TIMESTAMP,
//     //         end_date        TIMESTAMP,
//     //         created_at      TIMESTAMP NOT NULL,
//     //         updated_at      TIMESTAMP NOT NULL
//     //     )");
//     // }
// }
