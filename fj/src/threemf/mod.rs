use std::{
    fs::File,
    io::{self, prelude::*},
    path::PathBuf,
};

use thiserror::Error;

use tracing::info;

use zip::{result::ZipError, write::FileOptions, ZipWriter};

use crate::Mesh;

/// Export mesh to 3MF file
///
/// See [3MF specification] and [Open Packaging Conventions].
///
/// [3MF specification]: https://3mf.io/specification/
/// [Open Packaging Conventions]: https://standards.iso.org/ittf/PubliclyAvailableStandards/c061796_ISO_IEC_29500-2_2012.zip
pub fn export(_mesh: &Mesh, path: PathBuf) -> Result<(), Error> {
    info!("Exporting to `{}`", path.display());

    let file = File::create(&path)?;
    let mut archive = ZipWriter::new(file);

    archive.start_file("[Content_Types].xml", FileOptions::default())?;
    archive.write_all(include_bytes!("content-types.xml"))?;

    archive.start_file("_rels/.rels", FileOptions::default())?;
    archive.write_all(include_bytes!("rels.xml"))?;

    archive.start_file("3D/model.model", FileOptions::default())?;
    write!(
        archive,
        "\
<?xml version=\"1.0\" encoding=\"utf-8\"?>
<model
    xmlns=\"http://schemas.microsoft.com/3dmanufacturing/core/2015/02\"
    unit=\"millimeter\"
    xml:lang=\"en-US\">

    <resources>
    </resources>
    <build>
    </build>
</model>
    "
    )?;

    archive.finish()?;

    // TASK: Export model to 3MF file.
    todo!()
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Expected path to file, got `{0}`")]
    NoFileName(PathBuf),

    #[error("I/O error")]
    Io(#[from] io::Error),

    #[error("Zip error")]
    Zip(#[from] ZipError),
}
