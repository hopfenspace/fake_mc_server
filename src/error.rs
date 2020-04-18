use std::io;
use std::fmt;
use std::fmt::Display;

pub enum FakeMcError
{
	IoError(io::Error),
	OzelotError(ozelot::errors::Error),
}

impl Display for FakeMcError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		match self
		{
			FakeMcError::IoError(e) => write!(f, "{}", e),
			FakeMcError::OzelotError(e) => write!(f, "{}", e),
		}
	}
}

impl From<io::Error> for FakeMcError
{
	fn from(val: io::Error) -> Self
	{
		return FakeMcError::IoError(val);
	}
}

impl From<ozelot::errors::Error> for FakeMcError
{
	fn from(val: ozelot::errors::Error) -> Self
	{
		return FakeMcError::OzelotError(val);
	}
}
