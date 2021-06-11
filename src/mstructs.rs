
pub struct Expression<'a> {
    pub vars: Vec<&'a str>,
    pub formula: &'a str,
}