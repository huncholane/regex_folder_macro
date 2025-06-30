use regex_macro::load_regex_files;
load_regex_files!("./tests/regex");

#[test]
pub fn events_vec() {
    let events = EventRE::vec_from_file("./tests/samples/events.txt");
    assert!(events.unwrap().len() == 13);
}

#[test]
pub fn events_iter() {
    let events = EventRE::iter_from_file("./tests/samples/events.txt").unwrap();
    assert_eq!(events.count(), 13);
}
