use opentracingrust::Error;
use thrift;


pub fn thrift_error(error: thrift::Error) -> Error {
    Error::Msg(format!("{:?}", error))
}
