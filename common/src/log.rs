use std::fmt;

use bytes::Bytes;

pub type Timestamp = u64;

#[derive(Debug, Clone)]
pub struct Log {
    pub time: Timestamp,
    pub original_data: Bytes,
    // pub value: serde_json::Value,
    pub size_in_bytes: usize,
}

#[derive(Debug)]
pub enum ParseLogErrorCode {
    ParseError,
    NotAnObject,
    TimeColumnMissing,
    TimeColumnHasWrongFormat,
}

impl fmt::Display for ParseLogErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}] {}", self.code, self.message)
    }
}

#[derive(Debug)]
pub struct ParseLogErr {
    pub message: String,
    pub code: ParseLogErrorCode,
}

impl TryFrom<&[u8]> for Log {
    type Error = ParseLogErr;

    fn try_from(bytes: &[u8]) -> Result<Self, ParseLogErr> {
        let value: serde_json::Result<serde_json::Value> = serde_json::from_slice(bytes);

        match value {
            Err(error) => Err(ParseLogErr {
                message: format!(
                    "Could not parse json. Log: '{}'. Error: '{}'",
                    String::from_utf8_lossy(bytes),
                    error,
                ),
                code: ParseLogErrorCode::ParseError,
            }),
            Ok(value) => {
                let time = value
                    .as_object()
                    .ok_or(ParseLogErr {
                        message: String::from("Payload is not an object"),
                        code: ParseLogErrorCode::NotAnObject,
                    })?
                    .get("time")
                    .ok_or(ParseLogErr {
                        message: String::from("'time' column is missing"),
                        code: ParseLogErrorCode::TimeColumnMissing,
                    })?
                    .as_u64()
                    .ok_or(ParseLogErr {
                        message: String::from("'time' column has wrong format"),
                        code: ParseLogErrorCode::TimeColumnHasWrongFormat,
                    })?;

                Ok(Log {
                    time,
                    original_data: Bytes::copy_from_slice(bytes),
                    size_in_bytes: bytes.len(),
                })
            }
        }
    }
}
