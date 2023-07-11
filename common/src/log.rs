use bytes::Bytes;
use thiserror::Error;

pub type Timestamp = u64;

#[derive(Debug, Clone)]
pub struct Log {
    pub time: Timestamp,
    pub original_data: Bytes,
    // pub value: serde_json::Value,
    pub size_in_bytes: usize,
}

#[derive(Error, Debug)]
pub enum ParseLogErr {
    #[error("Could not parse json. Log: '{log:?}'. Error: '{error:?}'")]
    ParseError { log: String, error: String },
    #[error("Payload is not an object")]
    NotAnObject,
    #[error("'time' column is missing")]
    TimeColumnMissing,
    #[error("'time' column has wrong format")]
    TimeColumnHasWrongFormat,
}

impl TryFrom<&[u8]> for Log {
    type Error = ParseLogErr;

    fn try_from(bytes: &[u8]) -> Result<Self, ParseLogErr> {
        let value: serde_json::Result<serde_json::Value> = serde_json::from_slice(bytes);

        match value {
            Err(error) => Err(ParseLogErr::ParseError {
                log: String::from_utf8_lossy(bytes).into(),
                error: format!("{}", error),
            }),
            Ok(value) => {
                let time = value
                    .as_object()
                    .ok_or(ParseLogErr::NotAnObject)?
                    .get("time")
                    .ok_or(ParseLogErr::TimeColumnMissing)?
                    .as_u64()
                    .ok_or(ParseLogErr::TimeColumnHasWrongFormat)?;

                Ok(Log {
                    time,
                    original_data: Bytes::copy_from_slice(bytes),
                    size_in_bytes: bytes.len(),
                })
            }
        }
    }
}
