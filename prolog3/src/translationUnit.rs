#![allow(nonstandard_style)]
use std::{fs, io::prelude::*};

pub struct TranslationUnit {
    content: Vec<u8>,
}

impl TranslationUnit {
    pub fn fromFile<PathType>(path: PathType) -> Option<Self>
    where
    PathType: AsRef<std::path::Path>
    {
        let mut file = fs::File::open(path).ok()?;
        let mut content = Vec::new();

        file.read_to_end(&mut content).ok()?;

        Some(Self {
            content: content
        })
    }

    pub fn text(&self) -> &[u8] {
        &self.content
    }

    pub fn fromLiteral(text: &[u8]) -> Self {
        Self { content: text.to_owned() }
    }
}