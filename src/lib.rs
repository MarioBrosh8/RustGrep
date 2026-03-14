pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        //uses .lines rust builtin method to read each line
        if line.contains(query) {
            results.push(line);
        }
    }
    results //return value 
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)] //Markup that stablishes the beginning of the test
mod tests {
    //Markup that defines the test
    use super::*; //Grants access to the "Parent function" meaning the complete file

    #[test] //Markup that defines de beginning of the test function
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents)); //asserteq macro 
        //tests the two
        //parameters provided
        //are strictly equal
        //or returns an error
        //with detailed
        //information on why
        //it failed.
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
