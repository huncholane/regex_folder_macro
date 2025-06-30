use regex_macro::load_regex_files;

#[test]
pub fn events() {
    load_regex_files!("./tests/regex");
    let events = EventRE::vec_from_file("./tests/samples/events.txt");
    assert!(events.unwrap().len() == 13)
}
