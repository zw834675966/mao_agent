use crate::error::{Result, VectorError};
use crate::vector::index::VectorIndex;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// New snapshot files start with this 8-byte magic.
const MAGIC: &[u8; 8] = b"MAOVS01\0";

/// Identity envelope stored after MAGIC + header_len.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct SnapshotIdentity {
    pub model: String,
    pub dimension: usize,
}

/// MAGIC + u32le header_len + bincode(identity) + bincode(VectorIndex).
pub(crate) fn encode_snapshot(identity: &SnapshotIdentity, index: &VectorIndex) -> Result<Vec<u8>> {
    let header =
        bincode::serialize(identity).map_err(|e| VectorError::Serialization(e.to_string()))?;
    let index_bytes =
        bincode::serialize(index).map_err(|e| VectorError::Serialization(e.to_string()))?;
    let header_len = u32::try_from(header.len()).map_err(|_| {
        VectorError::Serialization("snapshot identity header exceeds u32 length".into())
    })?;

    let mut out = Vec::with_capacity(MAGIC.len() + 4 + header.len() + index_bytes.len());
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&header_len.to_le_bytes());
    out.extend_from_slice(&header);
    out.extend_from_slice(&index_bytes);
    Ok(out)
}

/// Load a headered envelope or a legacy unheadered VectorIndex bincode file.
pub(crate) fn load_snapshot(path: &Path) -> Result<(Option<SnapshotIdentity>, VectorIndex)> {
    let bytes = std::fs::read(path)?;
    decode_snapshot(&bytes)
}

fn decode_snapshot(bytes: &[u8]) -> Result<(Option<SnapshotIdentity>, VectorIndex)> {
    if let Some(rest) = bytes.strip_prefix(MAGIC) {
        if rest.len() < 4 {
            return Err(VectorError::IndexCorrupted(
                "truncated snapshot identity header".into(),
            ));
        }
        let header_len = u32::from_le_bytes([rest[0], rest[1], rest[2], rest[3]]) as usize;
        let rest = &rest[4..];
        if rest.len() < header_len {
            return Err(VectorError::IndexCorrupted(format!(
                "snapshot header length {header_len} exceeds remaining {} bytes",
                rest.len()
            )));
        }
        let identity: SnapshotIdentity = bincode::deserialize(&rest[..header_len])
            .map_err(|e| VectorError::Deserialization(e.to_string()))?;
        let index: VectorIndex = bincode::deserialize(&rest[header_len..])
            .map_err(|e| VectorError::Deserialization(e.to_string()))?;
        Ok((Some(identity), index))
    } else {
        let index: VectorIndex =
            bincode::deserialize(bytes).map_err(|e| VectorError::Deserialization(e.to_string()))?;
        Ok((None, index))
    }
}

/// Unique tmp sibling `{filename}.tmp.{pid}` — not `Path::with_extension("tmp")`.
fn tmp_sibling(dest: &Path) -> PathBuf {
    let pid = std::process::id();
    match dest.file_name() {
        Some(name) => {
            let mut tmp_name = name.to_os_string();
            tmp_name.push(format!(".tmp.{pid}"));
            dest.with_file_name(tmp_name)
        }
        None => dest.join(format!(".tmp.{pid}")),
    }
}

/// Write `bytes` then replace `dest`. Never `remove_file(dest)` first.
pub(crate) fn atomic_replace(dest: &Path, bytes: &[u8]) -> Result<()> {
    let tmp = tmp_sibling(dest);
    std::fs::write(&tmp, bytes)?;
    if let Err(e) = replace_file(&tmp, dest) {
        let _ = std::fs::remove_file(&tmp);
        return Err(e.into());
    }
    Ok(())
}

#[cfg(windows)]
fn replace_file(tmp: &Path, dest: &Path) -> std::io::Result<()> {
    use std::os::windows::ffi::OsStrExt;

    const MOVEFILE_REPLACE_EXISTING: u32 = 0x0000_0001;
    const MOVEFILE_WRITE_THROUGH: u32 = 0x0000_0008;

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn MoveFileExW(existing: *const u16, new: *const u16, flags: u32) -> i32;
    }

    fn wide_path(path: &Path) -> Vec<u16> {
        path.as_os_str()
            .encode_wide()
            .chain(std::iter::once(0))
            .collect()
    }

    let src = wide_path(tmp);
    let dst = wide_path(dest);
    let ok = unsafe {
        MoveFileExW(
            src.as_ptr(),
            dst.as_ptr(),
            MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
        )
    };
    if ok == 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(not(windows))]
fn replace_file(tmp: &Path, dest: &Path) -> std::io::Result<()> {
    std::fs::rename(tmp, dest)
}
