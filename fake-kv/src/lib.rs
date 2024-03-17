wit_bindgen::generate!({
    world: "hello",
    exports: {
        "wasi:keyvalue/atomic": FakeKV,
        "wasi:logging/logging": FakeKV,
    },
});

use exports::wasi::keyvalue::atomic::Guest;
use exports::wasi::logging::logging::Guest as LoggingGuest;
use wasi::filesystem::preopens::get_directories;
use wasi::filesystem::types::{DescriptorFlags, OpenFlags, PathFlags};
use wasi::keyvalue::types::*;

struct FakeKV;

impl Guest for FakeKV {
    fn increment(_bucket: &Bucket, key: Key, delta: u64) -> Result<u64, Error> {
        let dir = get_directories()
            .into_iter()
            .next()
            .map(|(d, _)| d)
            .expect("Should have a directory");
        let file = dir
            .open_at(
                PathFlags::empty(),
                &key,
                OpenFlags::CREATE,
                DescriptorFlags::READ | DescriptorFlags::WRITE,
            )
            .expect("Should be able to access file");
        let (data, _) = file.read(1, 0).expect("Should be able to read file");
        let mut count = data.into_iter().next().unwrap_or(0);
        count += delta as u8;
        file.write(&[count], 0)
            .expect("Should be able to write file");
        Ok(count as u64)
    }

    fn compare_and_swap(_bucket: &Bucket, _key: Key, _old: u64, _new: u64) -> Result<bool, Error> {
        unimplemented!("Not implemented for a demo")
    }
}

impl LoggingGuest for FakeKV {
    fn log(
        level: exports::wasi::logging::logging::Level,
        context: wit_bindgen::rt::string::String,
        message: wit_bindgen::rt::string::String,
    ) {
        println!(
            "Log level: {:?}, Context: {}, Message: {}",
            level, context, message
        );
    }
}
