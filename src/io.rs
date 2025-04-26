use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Result, Write};
use std::path::{Path, PathBuf};

/// Extensions for IO readers
pub trait ReadExt: Read {
    /// Read all bytes into a String
    fn read_string(&mut self) -> Result<String> {
        let mut string = String::new();
        self.read_to_string(&mut string)?;
        Ok(string)
    }
    
    /// Read all bytes into a Vec<u8>
    fn read_bytes(&mut self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        self.read_to_end(&mut bytes)?;
        Ok(bytes)
    }
    
    /// Read exactly n bytes into a buffer
    fn read_exact_vec(&mut self, n: usize) -> Result<Vec<u8>> {
        let mut buffer = vec![0; n];
        self.read_exact(&mut buffer)?;
        Ok(buffer)
    }
    
    /// Convert any reader to a buffered reader
    fn buffered(self) -> BufReader<Self>
    where
        Self: Sized,
    {
        BufReader::new(self)
    }
}

impl<R: Read> ReadExt for R {}

/// Extensions for IO writers
pub trait WriteExt: Write {
    /// Write a string and flush
    fn write_and_flush(&mut self, s: &str) -> Result<()> {
        self.write_all(s.as_bytes())?;
        self.flush()?;
        Ok(())
    }
    
    /// Convert any writer to a buffered writer
    fn buffered(self) -> BufWriter<Self>
    where
        Self: Sized,
    {
        BufWriter::new(self)
    }
}

impl<W: Write> WriteExt for W {}

/// Utility functions for file operations
pub struct FileUtils;

impl FileUtils {
    /// Read file contents as string
    pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
        fs::read_to_string(path)
    }
    
    /// Read file contents as bytes
    pub fn read_to_bytes<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
        fs::read(path)
    }
    
    /// Read file line by line
    pub fn read_lines<P: AsRef<Path>>(path: P) -> Result<impl Iterator<Item = Result<String>>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(reader.lines())
    }
    
    /// Write string to file
    pub fn write_string<P: AsRef<Path>>(path: P, contents: &str) -> Result<()> {
        fs::write(path, contents)
    }
    
    /// Write bytes to file
    pub fn write_bytes<P: AsRef<Path>>(path: P, bytes: &[u8]) -> Result<()> {
        fs::write(path, bytes)
    }
    
    /// Append string to file
    pub fn append_string<P: AsRef<Path>>(path: P, contents: &str) -> Result<()> {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        file.write_all(contents.as_bytes())?;
        Ok(())
    }
    
    /// Create all parent directories of a path if they don't exist
    pub fn ensure_parent_dirs<P: AsRef<Path>>(path: P) -> Result<()> {
        if let Some(parent) = path.as_ref().parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        Ok(())
    }
    
    /// Walk directory recursively and collect all file paths
    pub fn walk_dir<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        if path.as_ref().is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    files.append(&mut Self::walk_dir(&path)?);
                } else {
                    files.push(path);
                }
            }
        }
        Ok(files)
    }
}

/// A temporary file that is automatically deleted when it goes out of scope
pub struct TempFile {
    path: PathBuf,
}

impl TempFile {
    /// Create a new temporary file with optional content
    pub fn new(content: Option<&str>) -> Result<Self> {
        let mut path = std::env::temp_dir();
        path.push(format!("tmp-{}", uuid()));
        
        if let Some(content) = content {
            fs::write(&path, content)?;
        } else {
            File::create(&path)?;
        }
        
        Ok(Self { path })
    }
    
    /// Get the path to the temporary file
    pub fn path(&self) -> &Path {
        &self.path
    }
    
    /// Open the temporary file for reading
    pub fn open_read(&self) -> Result<File> {
        File::open(&self.path)
    }
    
    /// Open the temporary file for writing
    pub fn open_write(&self) -> Result<File> {
        File::create(&self.path)
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

// Helper to generate a simple UUID-like string
fn uuid() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("{:x}", now)
}
