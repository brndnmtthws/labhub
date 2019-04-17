use std::fs::File;
use std::io::prelude::*;
use std::panic;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};

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

static LOGGER_INITIALIZED: AtomicBool = AtomicBool::new(false);

fn setup() {
    if !LOGGER_INITIALIZED.compare_and_swap(false, true, Ordering::Relaxed) {
        env_logger::try_init().expect("Error initializing logger");
    }
}

pub fn run_test<T>(test: T)
where
    T: FnOnce() -> () + panic::UnwindSafe,
{
    setup();

    let result = panic::catch_unwind(test);

    assert!(result.is_ok())
}
