use sqlite;

pub struct DAO {
    conn: sqlite::Connection,
}

impl DAO {
    pub fn new(db_file: &str) -> sqlite::Result<DAO> {
        let conn = sqlite::open(db_file)?;

        conn.execute(
            "\
            CREATE TABLE IF NOT EXISTS todos (\
                id INTEGER PRIMARY KEY AUTOINCREMENT,\
                content TEXT,\
                checked INTEGER\
            )\
        ",
        )?;

        Ok(DAO { conn })
    }

    pub fn create_todo(&self, content: &str) -> sqlite::Result<()> {
        let mut statement = self
            .conn
            .prepare("INSERT INTO todos(content, checked) VALUES(?, 0)")?;
        statement.bind(1, content)?;
        statement.next()?;
        Ok(())
    }
}
