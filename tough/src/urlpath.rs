//! This module contains utilities for mapping URL paths to local Paths.

/// Converts a file URL into a file path.
/// `url.to_file_path()` will decode any percent encoding, which could restore path traversal
/// characters.
pub trait SafeUrlPath {
    /// Returns the path component of a URL as a filesystem path.
    fn safe_url_path(&self) -> &str;
}

#[cfg(windows)]
impl SafeUrlPath for url::Url {
    fn safe_url_path(&self) -> &str {
        let url_path = self.path();

        // Windows filepaths when written as `file://` URLs have path components prefixed with a /.
        if let Some(stripped) = url_path.strip_prefix('/') {
            stripped
        } else {
            url_path
        }
    }
}

#[cfg(unix)]
impl SafeUrlPath for url::Url {
    fn safe_url_path(&self) -> &str {
        self.path()
    }
}
