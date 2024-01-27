use anyhow::{anyhow, Result};
#[derive(Debug)]
pub struct Word {
    pub word: String,
    pub phonetic: String,
    pub definition: Vec<String>,
    pub traslation: Vec<String>,
    pub tags: String,
    pub exchanges: Vec<String>,
}
impl Default for Word {
    fn default() -> Self {
        Word {
            word: String::new(),
            phonetic: String::new(),
            definition: Vec::new(),
            traslation: Vec::new(),
            tags: String::new(),
            exchanges: Vec::new(),
        }
    }
}

impl Word {
    pub fn new() -> Self {
        Self::default()
    }
}

pub fn mock_word() -> Result<Word> {
    let mut word = Word::new();
    word.word = "abandon".to_string();
    word.phonetic = "ә'bændәn".to_string();
    word.tags=format!("{} {} {} {} {} {}","高考","CET4","CET6","托福","GRE","雅思");

    word.definition = vec![
        "n. the trait of lacking restraint or control; reckless freedom from inhibition or worry"
            .to_string(),
        "v. forsake, leave behind".to_string(),
        "v. give up with the intent of never claiming again".to_string(),
        "v. stop maintaining or insisting on; of ideas or claims".to_string(),
    ];

    word.exchanges = vec![
        "过去分词: abandoned".to_string(),
        "过去式: abandoned".to_string(),
        "现在分词: abandoning".to_string(),
        "第三人称单数: abandons".to_string(),
        "名词复数: abandons".to_string(),
    ];

    word.traslation = vec![
        "vt. 放弃, 抛弃, 遗弃, 使屈从, 沉溺, 放纵".to_string(),
        "n. 放任, 无拘束, 狂热".to_string(),
    ];

    // Err(anyhow!("get word is failed"))
    Ok(word)
}
