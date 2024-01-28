pub struct DBWord {
    pub word: String,
    pub phonetic: String,
    pub definition: String,
    pub translation: String,
    pub tag: String,
    pub exchange: String,
}
impl DBWord {
    pub fn new() -> Self {
        Self {
            word: String::new(),
            phonetic: String::new(),
            definition: String::new(),
            translation: String::new(),
            tag: String::new(),
            exchange: String::new(),
        }
    }
}
