use crate::search::{search, search_case_insensitive};

#[test]
fn case_sensitive() {
    let query = "rUst";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(vec!["Rust:"], search_case_insensitive(query, contents));
}

#[test]
fn one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
}
