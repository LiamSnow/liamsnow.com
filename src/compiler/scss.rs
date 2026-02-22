use crate::indexer::{FileSlot, Slots};
use std::io::{self, ErrorKind};
use std::path::Path;
use typst::syntax::{FileId, VirtualPath};

/// Make `Slots` VFS for Grass
#[derive(Debug)]
pub struct GrassSlotsFs<'a>(pub &'a Slots);

impl<'a> grass::Fs for GrassSlotsFs<'a> {
    fn is_dir(&self, _: &Path) -> bool {
        false
    }

    fn is_file(&self, path: &Path) -> bool {
        self.slot(path).is_some()
    }

    fn read(&self, path: &Path) -> io::Result<Vec<u8>> {
        match self.slot(path) {
            Some(slot) => Ok(slot.file.to_vec()),
            None => Err(ErrorKind::NotFound.into()),
        }
    }
}

impl<'a> GrassSlotsFs<'a> {
    fn slot(&self, path: &Path) -> Option<&FileSlot> {
        let vp = VirtualPath::new(path);
        let id = FileId::new(None, vp);
        self.0.get(&id)
    }
}
