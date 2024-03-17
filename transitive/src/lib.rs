use exports::wasi::keyvalue::{
    types::{
        GuestBucket, GuestIncomingValue, GuestOutgoingValue, IncomingValueAsyncBody,
        IncomingValueSyncBody, OutgoingValueBodyAsync, OutgoingValueBodySync, OwnBucket, OwnError,
        OwnIncomingValue, OwnOutgoingValue,
    },
    wasi_keyvalue_error::GuestError,
};

wit_bindgen::generate!({
    world: "hello",
    exports: {
        "wasi:keyvalue/types/bucket": FakeBucket,
        "wasi:keyvalue/types/incoming-value": FakeIncoming,
        "wasi:keyvalue/types/outgoing-value": FakeOutgoing,
        "wasi:keyvalue/wasi-keyvalue-error/error": FakeError,
    },
});

pub struct FakeBucket;

impl GuestBucket for FakeBucket {
    fn open_bucket(
        _name: wit_bindgen::rt::string::String,
    ) -> Result<exports::wasi::keyvalue::types::OwnBucket, exports::wasi::keyvalue::types::OwnError>
    {
        Ok(OwnBucket::new(FakeBucket))
    }
}

pub struct FakeIncoming;

impl GuestIncomingValue for FakeIncoming {
    fn incoming_value_consume_sync(
        _this: OwnIncomingValue,
    ) -> Result<IncomingValueSyncBody, OwnError> {
        todo!()
    }

    fn incoming_value_consume_async(
        _this: OwnIncomingValue,
    ) -> Result<IncomingValueAsyncBody, OwnError> {
        todo!()
    }

    fn incoming_value_size(&self) -> Result<u64, OwnError> {
        todo!()
    }
}

pub struct FakeOutgoing;

impl GuestOutgoingValue for FakeOutgoing {
    fn new_outgoing_value() -> OwnOutgoingValue {
        todo!()
    }

    fn outgoing_value_write_body_async(&self) -> Result<OutgoingValueBodyAsync, OwnError> {
        todo!()
    }

    fn outgoing_value_write_body_sync(
        &self,
        _value: OutgoingValueBodySync,
    ) -> Result<(), OwnError> {
        todo!()
    }
}

pub struct FakeError;

impl GuestError for FakeError {
    fn trace(&self) -> wit_bindgen::rt::string::String {
        "Fake error".to_string()
    }
}
