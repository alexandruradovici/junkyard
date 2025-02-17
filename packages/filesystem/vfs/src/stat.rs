use std::fmt;

#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
#[non_exhaustive]
pub enum Kind {
    File,
    Folder,
    Link,
    Unknown,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Kind::File => "file",
                Kind::Folder => "folder",
                Kind::Link => "link",
                Kind::Unknown => "unknown",
            }
        )
    }
}

pub struct Stat {
    pub kind: Kind,
    pub size: u64,
}

impl Stat {
    pub fn new(kind: Kind, size: u64) -> Stat {
        Stat { kind, size }
    }

    pub fn kind(&self) -> Kind {
        self.kind
    }

    pub fn size(&self) -> u64 {
        self.size
    }
}
