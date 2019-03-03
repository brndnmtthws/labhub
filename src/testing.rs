use std::fs::File;
use std::io::prelude::*;
use std::panic;
use std::path::PathBuf;

pub fn read_testdata_to_string(filename: &str) -> String {
    let mut datapath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    datapath.push("src/testdata");
    datapath.push(filename);

    let mut file = File::open(datapath).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");
    contents
}

fn setup() {
    env_logger::init();
}

pub fn run_test<T>(test: T)
where
    T: FnOnce() -> () + panic::UnwindSafe,
{
    setup();

    let result = panic::catch_unwind(test);

    assert!(result.is_ok())
}
