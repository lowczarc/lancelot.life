use mysql::Pool;

pub fn mysql_connection() -> Pool {
    let pool = mysql::Pool::new(concat!(
        "mysql://",
        env!("MYSQL_USER"),
        ":",
        env!("MYSQL_PASSWORD"),
        "@localhost:3306/",
        env!("MYSQL_DATABASE"),
    )).expect("Failed to connect with MYSQL");

    println!("Connected to MYSQL");

    pool.prep_exec(r"CREATE TABLE IF NOT EXISTS articles (
                        id int unique not null auto_increment,
                        titre varchar(255) not null,
                        date datetime not null,
                        content text
            )", ()).unwrap();

    pool.prep_exec(r"CREATE TABLE IF NOT EXISTS tags (
                        id int unique not null auto_increment,
                        article_id int not null,
                        tag varchar(255) not null
            )", ()).unwrap();

    pool
}