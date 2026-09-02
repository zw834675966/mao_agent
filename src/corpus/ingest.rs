use crate::corpus::parser::MarkdownParser;
use crate::error::{Result, VectorError};
use crate::model::Document;
use std::path::{Path, PathBuf};
use tracing::{info, warn};

/// Corpus file scanner and batch loader.
pub struct CorpusScanner;

impl CorpusScanner {
    /// Recursively scan a directory for all markdown files (`.md`, `.markdown`).
    pub fn scan_dir<P: AsRef<Path>>(dir: P) -> Result<Vec<PathBuf>> {
        let root = dir.as_ref();
        if !root.exists() {
            return Err(VectorError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Directory does not exist: {}", root.display()),
            )));
        }

        let mut files = Vec::new();
        Self::collect_markdown_files(root, &mut files)?;
        files.sort();
        Ok(files)
    }

    fn collect_markdown_files(dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
        if dir.is_dir() {
            for entry in std::fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    Self::collect_markdown_files(&path, files)?;
                } else if let Some(ext) = path.extension()
                    && (ext == "md" || ext == "markdown")
                {
                    files.push(path);
                }
            }
        }
        Ok(())
    }

    /// Load all documents from a directory.
    pub fn load_documents_from_dir<P: AsRef<Path>>(dir: P) -> Result<Vec<Document>> {
        let paths = Self::scan_dir(dir)?;
        info!(
            "Discovered {} markdown files in corpus directory",
            paths.len()
        );

        let mut docs = Vec::with_capacity(paths.len());
        for p in paths {
            match MarkdownParser::parse_file(&p) {
                Ok(doc) => docs.push(doc),
                Err(e) => {
                    warn!("Failed to parse document at {}: {}", p.display(), e);
                }
            }
        }

        info!("Successfully loaded {} valid documents", docs.len());
        Ok(docs)
    }
}
