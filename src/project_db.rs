//use postgres::Error;






// pub fn initialaize_db() -> Result<(), Error>{
//     let mut db = database::PgDatabase::new("postgres", "testtest", "localhost", "promandb")?;
//     let create_tables_statements: Vec<String> = create_tables!(
//         projects => {
//             id: primary,
//             title: string,
//             description: string(1000),
//             start_date: datetime,
//             end_date: datetime
//         },
//         notes => {
//             id: primary,
//             title: string,
//             body: string(1500)
//             created_at: datetime,
//             updated_at: datetime,
//             project_id: reference(projects)
//         }
//     );
//     match db.create_tables(create_tables_statements) {
//         Err(e) => println!("{:?}", e),
//         Ok(()) => println!("Database tables created successfully")
//     } 
// } 





// pub fn create_tables(&mut self) -> Result<(), Error> {
//     //let mut client = Client::connect("postgresql://postgres:testtest@localhost/promandb", NoTls)?;
//     self.client.batch_execute(
//         "
//         CREATE TABLE IF NOT EXISTS projects (
//             id              SERIAL PRIMARY KEY,
//             title           VARCHAR(255) NOT NULL,
//             description     VARCHAR(1200),
//             start_date      TIMESTAMP,
//             end_date        TIMESTAMP,
//             created_at      TIMESTAMP NOT NULL,
//             updated_at      TIMESTAMP NOT NULL
//         )
//         ",
//     )?;
//     self.client.batch_execute(
//         "
//             CREATE TABLE IF NOT EXISTS notes (
//                 id              SERIAL PRIMARY KEY,
//                 title           VARCHAR(255) NOT NULL,
//                 body            VARCHAR(1200),
//                 created_at      TIMESTAMP NOT NULL,
//                 updated_at      TIMESTAMP NOT NULL,
//                 project_id      INTEGER NOT NULL REFERENCES projects
//             )
//         ",
//     )?;

//     self.client.batch_execute(
//         "
//         CREATE TABLE IF NOT EXISTS todo_lists (
//             id              SERIAL PRIMARY KEY,
//             title           VARCHAR(255) NOT NULL,
//             created_at      TIMESTAMP NOT NULL,
//             updated_at      TIMESTAMP NOT NULL,
//             project_id      INTEGER NOT NULL REFERENCES projects
//         )
//         ",
//     )?;

//     self.client.batch_execute(
//         "
//             CREATE TABLE IF NOT EXISTS todo_items (
//                 id              SERIAL PRIMARY KEY,
//                 title           VARCHAR(255) NOT NULL,
//                 is_completed    BOOLEAN,
//                 created_at      TIMESTAMP NOT NULL,
//                 completed_at    TIMESTAMP NOT NULL,
//                 todo_list_id    INTEGER NOT NULL REFERENCES todo_lists
//             )
//             ",
//     )?;

//     self.client.batch_execute(
//         "
//             CREATE TABLE IF NOT EXISTS tags(
//                 id              SERIAL PRIMARY KEY,
//                 title           VARCHAR(255) NOT NULL,
//                 description     VARCHAR(1200),
//                 color           VARCHAR(50),
//                 project_id      INTEGER NOT NULL REFERENCES projects
//             )
//         ",
//     )?;

//     Ok(())
// }
