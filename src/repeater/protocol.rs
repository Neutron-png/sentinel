#![allow(dead_code)]

use crate::network::protocol::HttpVersion;

/// Protocol control options for the repeater
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolMode {
    /// Automatically preserve the original protocol
    Preserve,
    /// Force HTTP/1.1
    ForceHttp11,
    /// Force HTTP/2
    ForceHttp2,
    /// Force HTTP/3
    ForceHttp3,
}

impl ProtocolMode {
    pub const ALL: &'static [ProtocolMode] = &[
        Self::Preserve,
        Self::ForceHttp11,
        Self::ForceHttp2,
        Self::ForceHttp3,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            Self::Preserve => "Preserve",
            Self::ForceHttp11 => "HTTP/1.1",
            Self::ForceHttp2 => "HTTP/2",
            Self::ForceHttp3 => "HTTP/3",
        }
    }

    pub fn resolve_version(self, original: HttpVersion) -> HttpVersion {
        match self {
            Self::Preserve => original,
            Self::ForceHttp11 => HttpVersion::Http11,
            Self::ForceHttp2 => HttpVersion::Http2,
            Self::ForceHttp3 => HttpVersion::Http3,
        }
    }

    pub fn next(self) -> Self {
        let all = Self::ALL;
        let pos = all.iter().position(|e| *e == self).unwrap_or(0);
        all[(pos + 1) % all.len()]
    }

    pub fn prev(self) -> Self {
        let all = Self::ALL;
        let pos = all.iter().position(|e| *e == self).unwrap_or(0);
        all[(pos + all.len() - 1) % all.len()]
    }
}
