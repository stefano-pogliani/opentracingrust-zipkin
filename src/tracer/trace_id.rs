use std::io::Cursor;
use std::fmt;
use std::str::FromStr;

use byteorder::NetworkEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;

use data_encoding::HEXLOWER_PERMISSIVE;
use data_encoding::DecodeError;
use data_encoding::DecodeKind;

use rand::random;


/// Converts an 8-bytes buffer into an u64.
///
/// ```ignore
/// let buffer = [0, 0, 0, 0, 0, 0, 0, 1];
/// assert_eq!(buffer_to_u64(&buffer), 1);
/// ```
///
/// # Panics
///
/// If the give buffer is not 8 bytes.
fn buffer_to_u64(input: &[u8]) -> u64 {
    assert_eq!(8, input.len());
    let mut buffer = Cursor::new(input);
    buffer.read_u64::<NetworkEndian>().unwrap()
}


/// Converts an u64 into an 8-bytes buffer.
///
/// ```ignore
/// assert_eq!(u64_to_buffer(1), [0, 0, 0, 0, 0, 0, 0, 1]);
/// ```
fn u64_to_buffer(input: u64) -> Vec<u8> {
    let mut buffer = Vec::new();
    buffer.write_u64::<NetworkEndian>(input).unwrap();
    assert_eq!(buffer.len(), 8);
    buffer
}


/// Inner container for long or short trace ids.
#[derive(Clone, Debug, PartialEq)]
enum InnerID {
    Long([u8; 16]),
    Short([u8; 8]),
}


/// Zipkin trace identifier.
///
/// Zipkin trace ids can be short (8 bytes) or long (16 bytes).
/// By default, new ids are long trace id.
///
/// Identifiers can be converted to and decoded from strings.
#[derive(Clone, Debug, PartialEq)]
pub struct TraceID(InnerID);

impl TraceID {
    /// Create a TraceID for a (low, high) tuple.
    pub fn join(high: u64, low: u64) -> TraceID {
        let high = u64_to_buffer(high);
        let low = u64_to_buffer(low);
        let trace_id = [
            high[0], high[1], high[2], high[3], high[4], high[5], high[6], high[7],
            low[0], low[1], low[2], low[3], low[4], low[5], low[6], low[7]
        ];
        TraceID::from(trace_id)
    }

    /// Generate a new, random, 16 bytes ID.
    pub fn new() -> TraceID {
        TraceID(InnerID::Long(random::<[u8; 16]>()))
    }
}

impl TraceID {
    /// Returns the id as a (u64, u64) tuple.
    pub fn split(&self) -> (u64, u64) {
        match self.0 {
            InnerID::Long(ref id) => {
                let high = buffer_to_u64(&id[0..8]);
                let low = buffer_to_u64(&id[8..16]);
                (high, low)
            },
            InnerID::Short(ref id) => {
                let low = buffer_to_u64(&id[0..8]);
                (0, low)
            }
        }
    }
}

impl fmt::Display for TraceID {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let id: &[u8] = match self.0 {
            InnerID::Long(ref id) => id,
            InnerID::Short(ref id) => id
        };
        for byte in id {
            write!(fmt, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl FromStr for TraceID {
    type Err = DecodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match HEXLOWER_PERMISSIVE.decode_len(s.len()) {
            Ok(8) => {
                let mut buffer = [0; 8];
                HEXLOWER_PERMISSIVE
                    .decode_mut(s.as_bytes(), &mut buffer)
                    .map_err(|err| err.error)?;
                Ok(TraceID::from(buffer))
            },
            Ok(16) => {
                let mut buffer = [0; 16];
                HEXLOWER_PERMISSIVE
                    .decode_mut(s.as_bytes(), &mut buffer)
                    .map_err(|err| err.error)?;
                Ok(TraceID::from(buffer))
            },
            _ => Err(DecodeError {
                position: 0,
                kind: DecodeKind::Length
            })
        }
    }
}

impl From<[u8; 8]> for TraceID {
    fn from(id: [u8; 8]) -> TraceID {
        TraceID(InnerID::Short(id))
    }
}

impl From<[u8; 16]> for TraceID {
    fn from(id: [u8; 16]) -> TraceID {
        TraceID(InnerID::Long(id))
    }
}


#[cfg(test)]
mod tests {
    use super::InnerID;
    use super::TraceID;

    #[test]
    fn generate_long_id() {
        let id = TraceID::new();
        match id.0 {
            InnerID::Short(_) => panic!("Generated IDs should be long"),
            _ => ()
        }
    }

    #[test]
    fn id_si_random() {
        let id1 = TraceID::new();
        let id2 = TraceID::new();
        assert_ne!(id1, id2);
    }

    mod from_bytes {
        use super::super::TraceID;

        #[test]
        fn long_id() {
            let inner: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
            let id1 = TraceID::from(inner.clone());
            let id2 = TraceID::from(inner);
            assert_eq!(id1, id2);
        }

        #[test]
        fn short_id() {
            let inner: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
            let id1 = TraceID::from(inner.clone());
            let id2 = TraceID::from(inner);
            assert_eq!(id1, id2);
        }
    }

    mod from_string {
        use std::str::FromStr;
        use data_encoding::DecodeKind;
        use super::super::TraceID;

        #[test]
        fn long_id() {
            let id = TraceID::from_str("0102030405060708090a0b0c0d0e0f10").unwrap();
            let expected = TraceID::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
            assert_eq!(id, expected);
        }

        #[test]
        fn short_id() {
            let id = TraceID::from_str("090a0b0c0d0e0f10").unwrap();
            let expected = TraceID::from([9, 10, 11, 12, 13, 14, 15, 16]);
            assert_eq!(id, expected);
        }

        #[test]
        fn too_short() {
            match TraceID::from_str("deadbeef") {
                Err(err) => assert_eq!(err.kind, DecodeKind::Length),
                _ => panic!("String decoding should have failed")
            }
        }

        #[test]
        fn too_long() {
            match TraceID::from_str("deadbeef0102030405060708090a0b0c0d0e0f10") {
                Err(err) => assert_eq!(err.kind, DecodeKind::Length),
                _ => panic!("String decoding should have failed")
            }
        }
    }

    mod to_string {
        use super::super::TraceID;

        #[test]
        fn long_id() {
            let id = TraceID::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
            let id = id.to_string();
            assert_eq!("0102030405060708090a0b0c0d0e0f10", id);
        }

        #[test]
        fn short_id() {
            let id = TraceID::from([9, 10, 11, 12, 13, 14, 15, 16]);
            let id = id.to_string();
            assert_eq!("090a0b0c0d0e0f10", id);
        }
    }

    mod split_into_u64s {
        use super::super::TraceID;

        #[test]
        fn high_and_low() {
            let id = TraceID::from([0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 2]);
            let (high, low): (u64, u64) = id.split();
            assert_eq!(high, 5);
            assert_eq!(low, 2);
        }

        #[test]
        fn high_is_zero_when_low_is_enough() {
            let id = TraceID::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 55]);
            let (high, low): (u64, u64) = id.split();
            assert_eq!(high, 0);
            assert_eq!(low, 55);
        }

        #[test]
        fn high_and_low_for_short() {
            let id = TraceID::from([0, 0, 0, 0, 0, 0, 0, 55]);
            let (high, low): (u64, u64) = id.split();
            assert_eq!(high, 0);
            assert_eq!(low, 55);
        }
    }

    mod join_for_u64s {
        use super::super::TraceID;

        #[test]
        fn high_and_low() {
            let id = TraceID::join(5, 2);
            assert_eq!(id, TraceID::from([0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 2]));
        }

        #[test]
        fn high_is_zero_when_low_is_enough() {
            let id = TraceID::join(0, 2);
            assert_eq!(id, TraceID::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2]));
        }
    }
}
