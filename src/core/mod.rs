use std::fmt;

pub enum ErrorKind {
	FileNotFound(String),
	FileNotAllowed(String),
	FileExists(String)
}

pub struct Error {
	pub fatal:bool,
	pub kind:ErrorKind
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind { // FIXME Andi 2020-02-28: I don't understand why the & is here but not on the s (it doesn't like &s)
        	ErrorKind::FileNotFound(s) => write!(f, "File not found: {}", s.clone()),
        	ErrorKind::FileNotAllowed(s) => write!(f, "File permissions denied: {}", s.clone()),
        	ErrorKind::FileExists(s) => write!(f, "Already exists: {}", s.clone())
        }
    }
}

pub fn print_nonfatal(result: Result<()>) -> Result<bool> {
	if let Err(error) = result {
		if (error.fatal) {
			Err(error)
		} else {
			print!("Warning: {}", error);
			Ok(false)
		}
	} else {
		Ok(true)
	}
}

pub fn forgive_nonfatal(result: Result<()>) -> Result<bool> {
	if let Err(error) = result {
		if (error.fatal) {
			Err(error)
		} else {
			print!("{}", error);
			Ok(false)
		}
	} else {
		Ok(true)
	}
}
