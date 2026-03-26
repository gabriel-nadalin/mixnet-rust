use std::io::{self, BufReader, BufWriter, Write};
use std::fs::File;
use std::path::Path;
use serde::Serialize;
use serde::de::DeserializeOwned;


/// writes canonical JSON to `path`, optional pretty file if `pretty` feature enabled,
/// and optional signature to `path.sig` if provided.
pub fn write_json<T: Serialize, S: Serialize>(
    value: &T,
    path: impl AsRef<Path>,
    sig: Option<&S>,
) -> std::io::Result<()> {
    let path = path.as_ref();

    let json = serde_json_canonicalizer::to_vec(value)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    atomic_write(path, &json)?;

    #[cfg(feature = "pretty")]
    {
        let pretty = serde_json::to_vec_pretty(value)?;
        let pretty_path = {
            let stem = path.file_stem().unwrap_or_default();
            path.with_file_name(format!("{}_pretty.json", stem.to_string_lossy()))
        };
        atomic_write(&pretty_path, &pretty)?;
    }

    // assumes valid signature
    if let Some(s) = sig {
        let sig_json = serde_json_canonicalizer::to_vec(s)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let sig_path = {
            let stem = path.file_stem().unwrap_or_default();
            path.with_file_name(format!("{}.sig.json", stem.to_string_lossy()))
        };
        atomic_write(&sig_path, &sig_json)?;
    }

    Ok(())
}


pub fn read_json<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T, Box<dyn std::error::Error>> {
    let file = File::open(path.as_ref())
        .map_err(|e| format!("{}: {e}", path.as_ref().display()))?;
    Ok(serde_json::from_reader(BufReader::new(file))?)
}

pub fn request_user_input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn atomic_write(path: &Path, data: &[u8]) -> io::Result<()> {
    let tmp = path.with_extension("tmp");
    {
        let file = File::create(&tmp)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(data)?;
        writer.flush()?;
    }
    std::fs::rename(&tmp, path)?;
    Ok(())
}