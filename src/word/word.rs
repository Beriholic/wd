use color_eyre::eyre::Result;

use crate::db::query::query;
#[derive(Debug)]
pub struct Word {
    pub word_info: String,
    pub definition: Vec<String>,
    pub translation: Vec<String>,
    pub tags: String,
    pub exchanges: Vec<String>,
}
impl Default for Word {
    fn default() -> Self {
        Word {
            word_info: String::new(),
            definition: Vec::new(),
            translation: Vec::new(),
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

#[allow(dead_code)]
pub fn mock_word() -> Result<Word> {
    let mut word = Word::new();
    word.word_info = format!("{} [{}]", "abandon", "ә'bændәn");
    word.tags = format!(
        "{} {} {} {} {} {}",
        "高考", "CET4", "CET6", "托福", "GRE", "雅思"
    );

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

    word.translation = vec![
        "vt. 放弃, 抛弃, 遗弃, 使屈从, 沉溺, 放纵".to_string(),
        "n. 放任, 无拘束, 狂热".to_string(),
    ];

    Ok(word)
}
pub fn query_word(word_name: &str) -> Result<Word> {
    let mut res = query(word_name)?;

    if res.word_info.len() == 0 {
        res.word_info = String::from("单词未找到");
    }

    Ok(res)
}

#[cfg(test)]
mod test {
    use super::{mock_word, query_word};

    #[test]
    fn test_mock_word() {
        let _ = mock_word().expect("mock word is failed");
    }

    #[test]
    fn test_query_word() {
        query_word("hello").expect("query word is failed");
    }
}
