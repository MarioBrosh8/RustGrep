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

#[cfg(test)] //Markup that stablishes the beginning of the test
mod tests {
    //Markup that defines the test
    use super::*; //Grants access to the "Parent function" meaning the complete file
    #[test] //Markup that defines de beginning of the test function
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents)); //asserteq macro 
        //tests the two
        //parameters provided
        //are strictly equal
        //or returns an error
        //with detailed
        //information on why
        //it failed.
    }
}
