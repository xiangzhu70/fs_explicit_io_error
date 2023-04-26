
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FsExplicitIoError {
    #[error("IO error in function {function} for path: {path:?} - {source}")]
    IoError {
        source: std::io::Error,
        path: PathBuf,
        function: &'static str,
    },
}

fn wrap_io_error<T>(
    result: std::io::Result<T>,
    path: &Path,
    function: &'static str,
) -> Result<T, FsExplicitIoError> {
    result.map_err(|source| FsExplicitIoError::IoError {
        source,
        path: path.to_owned(),
        function,
    })
}

macro_rules! wrap_fs_fn {
    ($name:ident, $result:ty) => {
        fn $name<P: AsRef<Path>>(path: P) -> Result<$result, FsExplicitIoError> {
            let path_ref = path.as_ref();
            wrap_io_error(std::fs::$name(path_ref), path_ref, stringify!($name))
        }
    };
}

wrap_fs_fn!(read_to_string, String);

fn main() {
    let path = Path::new("foo.txt");
    let result = read_to_string(&path);
    match result {
        Ok(content) => println!("File content: {}", content),
        Err(err) => println!("Error: {}", err),
    }
}
