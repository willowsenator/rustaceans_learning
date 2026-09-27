use std::error::Error;
use std::{fmt, io};

#[derive(Debug)]
enum CopyError {
    In(io::Error),
    Out(io::Error),
}

impl fmt::Display for CopyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CopyError::In(_) => write!(f, "fail to read source"),
            CopyError::Out(_) => write!(f, "fail to write destination"),
        }
    }
}

impl Error for CopyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CopyError::In(e) | CopyError::Out(e) => Some(e),
        }
    }
}

fn read_source() -> Result<String, io::Error> {
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "source.txt not found",
    ))
}

fn write_destination(_contents: &str) -> Result<(), io::Error> {
    Err(io::Error::new(
        io::ErrorKind::PermissionDenied,
        "destination.txt is read-only",
    ))
}

fn copy_file() -> Result<(), CopyError> {
    let contents = read_source().map_err(CopyError::In)?;
    write_destination(&contents).map_err(CopyError::Out)?;
    Ok(())
}

fn main() {
    if let Err(err) = copy_file() {
        eprintln!("Error: {err}");
        let mut cause = err.source();
        while let Some(inner) = cause {
            eprintln!("Caused by: {inner}");
            cause = inner.source();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_file_reports_source_read_failure() {
        let result = copy_file();

        let err = result.expect_err("expected a CopyError");
        assert_eq!(err.to_string(), "fail to read source");
        let cause = err.source().expect("expected an underlying io::Error");
        assert_eq!(cause.to_string(), "source.txt not found");
    }

    #[test]
    fn write_destination_reports_write_failure() {
        let err = write_destination("data").expect_err("expected an io::Error");
        assert_eq!(err.to_string(), "destination.txt is read-only");

        let copy_err = CopyError::Out(err);
        assert_eq!(copy_err.to_string(), "fail to write destination");
    }
}
