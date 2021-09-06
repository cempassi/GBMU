use std::fmt;
use std::string::FromUtf8Error;
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    //InvalidCartridgeType(u8),
    //InvalidDestinationCode(u8),
    //InvalidCgbFlag(u8),
    //InvalidSgbFlag(u8),
    InvalidNewLicense(String),
    //InvalidOldLicenseCode(u8),
    //InvalidRamSize(u8),
    //InvalidRomSize(u8),
    InvalidUtf8(FromUtf8Error),
    InvalidParse(ParseIntError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // Error::InvalidCartridgeType(v) => write!(f, "Invalid cartridge type {:02x}", v),
            // Error::InvalidDestinationCode(v) => write!(f, "Invalid destination code {:02x}", v),
            // Error::InvalidCgbFlag(v) => write!(f, "Invalid cgb flag {:02x}", v),
            // Error::InvalidSgbFlag(v) => write!(f, "Invalid sgb flag {:02x}", v),
            Error::InvalidNewLicense(v) => write!(f, "Invalid new license code {}", v),
            // Error::InvalidOldLicenseCode(v) => write!(f, "Invalid old license code {:02x}", v),
            // Error::InvalidRamSize(v) => write!(f, "Invalid ram size {:02x}", v),
            // Error::InvalidRomSize(v) => write!(f, "Invalid rom size {:02x}", v),
            Error::InvalidUtf8(v) => {
                write!(f, "invalid utf8 for {:?}: {}", v.as_bytes(), v.utf8_error())
            }
            Error::InvalidParse(v) =>
                write!(f, "invalid parse for {:?}", v),
        }
    }
}

impl std::convert::From<FromUtf8Error> for Error {
    fn from(v: FromUtf8Error) -> Self {
        Self::InvalidUtf8(v)
    }
}

impl std::convert::From<ParseIntError> for Error {
    fn from(v: ParseIntError) -> Self {
        Self::InvalidParse(v)
    }
}
