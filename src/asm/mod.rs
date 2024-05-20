pub mod generator;
pub mod parser;
pub fn tokenize(i: String) -> Vec<String> {
    let v = i.split_whitespace().collect::<Vec<&str>>();
    v.iter().map(|x| x.to_string()).collect::<Vec<String>>()
}
