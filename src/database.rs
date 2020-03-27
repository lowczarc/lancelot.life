use mysql::Pool;

pub fn mysql_connection() -> Pool {
    let pool = mysql::Pool::new(concat!(
        "mysql://",
        env!("MYSQL_USER"),
        ":",
        env!("MYSQL_PASSWORD"),
        "@localhost:3306/",
        env!("MYSQL_DATABASE"),
    ))
    .expect("Failed to connect with MYSQL");

    println!("Connected to MYSQL");

    pool.prep_exec(
        r"CREATE TABLE IF NOT EXISTS goals (
            id int not null,
            content text not null
        )",
        (),
    )
    .unwrap();

    pool.prep_exec(
        r"CREATE TABLE IF NOT EXISTS influences (
            id int not null,
            name varchar(255) not null,
            link text
        )",
        (),
    )
    .unwrap();

    pool
}
