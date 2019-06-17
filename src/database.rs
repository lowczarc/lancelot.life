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
        r"CREATE TABLE IF NOT EXISTS articles (
            id int unique not null auto_increment,
            image varchar(255),
            titre varchar(255) not null,
            date datetime not null,
            last_modified datetime not null,
            public bool,
            content text
        )",
        (),
    )
    .unwrap();

    pool.prep_exec(
        r"CREATE TABLE IF NOT EXISTS projects (
            id int unique not null auto_increment,
            titre varchar(255) not null,
            date datetime not null,
            image text
        )",
        (),
    )
    .unwrap();

    pool.prep_exec(
        r"CREATE TABLE IF NOT EXISTS links (
            id int unique not null auto_increment,
            project_id int not null,
            type varchar(255) not null,
            link text not null
        )",
        (),
    )
    .unwrap();

    pool.prep_exec(
        r"CREATE TABLE IF NOT EXISTS tags (
            id int unique not null auto_increment,
            project_id int,
            article_id int,
            tag varchar(255) not null
        )",
        (),
    )
    .unwrap();

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
