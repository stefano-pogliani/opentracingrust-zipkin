use data_encoding;
use thrift;

use opentracingrust::Error;


pub fn data_encoding_error(error: data_encoding::DecodeError) -> Error {
    Error::Msg(format!("{:?}", error))
}

pub fn thrift_error(error: thrift::Error) -> Error {
    Error::Msg(format!("{:?}", error))
}
