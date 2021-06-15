use std::{fs::File, io, path::PathBuf};

use thiserror::Error;

use tracing::info;

use crate::Mesh;

/// Export mesh to 3MF file
///
/// See [3MF specification] and [Open Packaging Conventions Fundamentals].
///
/// [3MF specification]: https://3mf.io/specification/
/// [Open Packaging Conventions Fundamentals]: https://docs.microsoft.com/en-us/previous-versions/windows/desktop/opc/open-packaging-conventions-overview
pub fn export_3mf(_mesh: &Mesh, path: PathBuf) -> Result<(), Error> {
    let name = path
        .file_stem()
        .ok_or_else(|| Error::NoFileName(path.clone()))?
        .to_string_lossy();

    info!("Exporting \"{}\" to `{}`", name, path.display(),);

    let _file = File::create(&path)?;

    // TASK: Export model to 3MF file.
    todo!()
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Expected path to file, got `{0}`")]
    NoFileName(PathBuf),

    #[error("I/O error")]
    Io(#[from] io::Error),
}
