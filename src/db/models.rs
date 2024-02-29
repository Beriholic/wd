#[derive(Debug)]
pub struct Word {
    pub word_info: Option<String>,
    pub definition: Option<Vec<String>>,
    pub translation: Option<Vec<String>>,
    pub tags: Option<String>,
    pub exchanges: Option<Vec<String>>,
}

impl Word {
    pub fn new() -> Self {
        Word {
            word_info: None,
            definition: None,
            translation: None,
            tags: None,
            exchanges: None,
        }
    }
}
