use crate::error::RlError;

/// Prints all params to stderr
pub fn error(params: Vec<&str>) -> Result<(),RlError> {
    eprintln!("{:?}",params);
    Ok(())
}
