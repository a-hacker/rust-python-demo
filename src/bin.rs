use std::env;
use typo_checker::Dictionary;


fn main() {
    let mut args: Vec<String> = env::args().collect();
    let test_string: String = args.pop().unwrap();
    let root_dir: String = args.pop().unwrap();
    let dict = Dictionary::new(root_dir.as_str());

    let matches = dict.search(test_string.as_str());
    matches.into_iter().for_each(|s| println!("{}", s))
}
