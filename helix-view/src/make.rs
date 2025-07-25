use helix_lsp::lsp::DiagnosticSeverity;
use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct Location {
    pub path: PathBuf,
    pub line: usize,
}

// TODO(szulf): maybe add a way for entries to reference other entries
// so that things like note: can actually be linked back to the original error
#[derive(Debug, Clone)]
pub struct Entry {
    pub location: Location,
    pub msg: String,
    pub severity: DiagnosticSeverity,
}

impl Entry {
    pub fn new(location: Location, msg: &str, severity: DiagnosticSeverity) -> Self {
        Self {
            location: location,
            msg: msg.to_string(),
            severity: severity,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct List {
    entries: Vec<Entry>,
}

impl List {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    // TODO(szulf): change this to implementing the IntoIterator trait instead
    pub fn into_iter(self) -> impl Iterator<Item = Entry> {
        self.entries.into_iter()
    }

    pub fn set(&mut self, entries: Vec<Entry>) {
        self.entries = entries;
    }
}
