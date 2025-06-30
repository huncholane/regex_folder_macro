use regex_folder_macro::load_regex_files;

load_regex_files!("regex");

fn main() {
    let events = EventRE::vec_from_file("samples/events.txt").unwrap();
    println!("{}", serde_json::to_string_pretty(&events).unwrap());
}
