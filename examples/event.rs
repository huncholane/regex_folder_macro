use regex_macro::load_regex_files;
load_regex_files!("tests/regex");

fn main() {
    let events = EventRE::vec_from_file("tests/samples/events.txt").unwrap();
    println!("{}", serde_json::to_string_pretty(&events).unwrap());
}
