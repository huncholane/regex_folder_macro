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

#[test]
pub fn first_event() {
    let event = EventRE::from_file("samples/events.txt").unwrap().unwrap();
    assert_eq!(event.val, "7:30–8:00 AM	Wake up + hydrate + light stretch");
}

#[test]
pub fn first_event_captures() {
    let mut content = String::new();
    let captures = EventRE::captures_from_file(&mut content, "samples/events.txt")
        .unwrap()
        .unwrap();
    assert_eq!(
        captures.get(0).unwrap().as_str(),
        "7:30–8:00 AM	Wake up + hydrate + light stretch"
    );
    assert_eq!(captures.name("end").unwrap().as_str(), "8:00");
    assert_eq!(captures.name("end").unwrap().start(), 57);
}

#[test]
pub fn event_captures_iter() {
    let mut content = String::new();
    let captures_iter =
        EventRE::captures_iter_from_file(&mut content, "samples/events.txt").unwrap();
    assert_eq!(captures_iter.count(), 13);
}
