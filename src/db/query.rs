use lazy_static::lazy_static;
use std::{collections::HashMap, env};

use crate::word::word::Word;

use super::model::DBWord;
use rusqlite::{Connection, Result};

lazy_static! {
    static ref FIELD_MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("p", "过去式");
        m.insert("d", "过去分词");
        m.insert("i", "现在分词");
        m.insert("3", "第三人称单数");
        m.insert("r", "比较级");
        m.insert("t", "最高级");
        m.insert("s", "名词复数");

        m.insert("zk", "中考");
        m.insert("gk", "高考");
        m.insert("ky", "考研");
        m.insert("cet4", "CET4");
        m.insert("cet6", "CET6");
        m.insert("toefl", "托福");
        m.insert("ielts", "雅思");
        m.insert("gre", "GRE");

        m
    };
}
fn get_db_conn() -> Result<Connection> {
    let db_url = format!("{}/{}", env::var("HOME").unwrap(), ".config/wd/stardict.db");
    let conn = Connection::open(db_url)?;
    Ok(conn)
}

pub fn query(query_word: &str) -> Result<Word> {
    let conn = get_db_conn()?;
    let mut stmt = conn.prepare("SELECT * FROM stardict WHERE word=?1 limit 1")?;

    let res = stmt.query_map([query_word], |row| {
        Ok(DBWord {
            word: row.get(1)?,
            phonetic: row.get(3)?,
            definition: row.get(4)?,
            translation: row.get(5)?,
            tag: row.get(9)?,
            exchange: row.get(12)?,
        })
    })?;

    let mut db_word = DBWord::new();

    for i in res {
        db_word = i?;
    }

    let mut word = Word::new();

    if db_word.word.len() == 0 {
        return Ok(word);
    }

    word.word = db_word.word.to_string();
    word.phonetic = db_word.phonetic.to_string();
    word.definition = db_word
        .definition
        .split('\n')
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    word.translation = db_word
        .translation
        .split('\n')
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    if db_word.tag.len() != 0 {
        let mut tags = String::new();
        for tag in db_word.tag.split(' ') {
            tags.push_str(FIELD_MAP.get(tag).unwrap());
            tags.push_str(" ");
        }
        word.tags = tags;
    }
    if db_word.exchange.len() != 0 {
        let mut exchange = db_word
            .exchange
            .split('/')
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        for i in 0..exchange.len() {
            let mut s = exchange[i].clone();
            let mut chars = s.chars();
            let first_char = chars.next().unwrap();
            let mut first_char_str = String::new();
            first_char_str.push(first_char);
            let first_char_str = first_char_str.as_str();
            let first_char_str = FIELD_MAP.get(first_char_str).unwrap();
            s.replace_range(0..1, first_char_str);
            exchange[i] = s;
        }

        word.exchanges = exchange;
    }
    Ok(word)
}

#[cfg(test)]
mod test {
    use super::{get_db_conn, query};

    #[test]
    fn test_query() {
        let query_word = "test";
        let res = query(query_word).unwrap();
        assert_ne!(res.word.len(), 0);
    }

    #[test]
    fn test_query_not_found() {
        let query_word = "test_not_found";
        let res = query(query_word).unwrap();
        assert_eq!(res.word.len(), 0);
    }
    #[test]
    fn test_get_db_conn() {
        let _ = get_db_conn();
    }
}
