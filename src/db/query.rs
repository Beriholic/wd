use lazy_static::lazy_static;
use std::{collections::HashMap, env};

use crate::word::word::Word;

use super::model::DBWord;
use rusqlite::{Connection, Result};

lazy_static! {
    static ref FIELD_MAP_STR: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
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
    static ref FIELD_MAP_CHAR: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        m.insert('p', "过去式");
        m.insert('d', "过去分词");
        m.insert('i', "现在分词");
        m.insert('3', "第三人称单数");
        m.insert('r', "比较级");
        m.insert('t', "最高级");
        m.insert('s', "名词复数");
        m.insert('0', "原型");
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
            word: match row.get(1) {
                Err(_) => "".to_string(),
                Ok(res) => res,
            },
            phonetic: match row.get(3) {
                Err(_) => "".to_string(),
                Ok(res) => res,
            },
            definition: match row.get(4) {
                Err(_) => "".to_string(),
                Ok(res) => res,
            },
            translation: match row.get(5) {
                Err(_) => "".to_string(),
                Ok(res) => res,
            },
            // translation: row.get(5)?,
            tag: match row.get(9) {
                Err(_) => "".to_string(),
                Ok(res) => res,
            },
            exchange: match row.get(12) {
                Err(_) => "".to_string(),
                Ok(res) => res,
            },
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

    word.word_info = match db_word.word.len() {
        0 => word.word_info,
        _ => match db_word.phonetic.len() {
            0 => db_word.word,
            _ => format!("{} [{}]", db_word.word, db_word.phonetic),
        },
    };
    if db_word.definition.len() != 0 {
        word.definition = db_word
            .definition
            .split('\n')
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
    }
    if db_word.translation.len() != 0 {
        word.translation = db_word
            .translation
            .split('\n')
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
    }
    if db_word.tag.len() != 0 {
        let db_word_tag = db_word.tag.split_whitespace().collect::<Vec<&str>>();
        let mut tags = Vec::new();

        for item in db_word_tag.iter() {
            if let Some(v) = FIELD_MAP_STR.get(item) {
                tags.push(v.to_string());
            }
        }

        word.tags = tags.join(", ");
    }
    if db_word.exchange.len() != 0 {
        let exchanges = db_word
            .exchange
            .split('/')
            .filter(|x| x.chars().nth(0).unwrap() != '1')
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        for item in exchanges.iter() {
            let item = item.split(':').collect::<Vec<&str>>();
            let first_char = item[0].chars().nth(0).unwrap();

            if let Some(v) = FIELD_MAP_CHAR.get(&first_char) {
                let mut item = item.join(": ");
                item.replace_range(0..1, v);
                word.exchanges.push(item);
            }
        }
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
        assert_ne!(res.word_info.len(), 0);
    }

    #[test]
    fn test_query_not_found() {
        let query_word = "test_not_found";
        let res = query(query_word).unwrap();
        assert_eq!(res.word_info.len(), 0);
    }
    #[test]
    fn test_get_db_conn() {
        let _ = get_db_conn();
    }
    #[test]
    fn test_query_done() {
        let query_word = "done";
        let res = query(query_word).unwrap();
        assert_ne!(res.word_info.len(), 0);
    }
    #[test]
    fn test_query_buff() {
        let query_word = "buff";
        let res = query(query_word).unwrap();
        dbg!(&res);
        assert_ne!(res.word_info.len(), 0);
    }
}
