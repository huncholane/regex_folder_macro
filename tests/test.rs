use regex_folder_macro::load_regex_files;

load_regex_files!("./regex");

#[test]
pub fn events_vec() {
    let events = EventRE::vec_from_file("./samples/events.txt");
    assert!(events.unwrap().len() == 13);
}

#[test]
pub fn events_iter() {
    let events = EventRE::iter_from_file("./samples/events.txt").unwrap();
    assert_eq!(events.count(), 13);
}
