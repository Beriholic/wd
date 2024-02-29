use std::collections::HashMap;

use color_eyre::eyre::Result;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::entity::stardict::{Entity as DbWord, Model};
use crate::{db::init::db_connect, entity::stardict};
use lazy_static::lazy_static;

use super::models;

pub async fn query_word(qword: &str) -> Result<Option<models::Word>> {
    let db = db_connect().await?;

    let db_word = DbWord::find()
        .filter(stardict::Column::Word.eq(qword))
        .one(&db)
        .await?;

    return match db_word {
        Some(db_word) => Ok(Some(dbword_convert_word(db_word).await)), // Wrap the struct Word inside Some variant
        None => Ok(None),
    };
}

pub async fn dbword_convert_word(db_word: Model) -> models::Word {
    let mut word = models::Word::new();

    word.word_info = Some(format!(
        "{} [{}]",
        db_word.word,
        db_word.phonetic.unwrap_or_default()
    ));

    word.definition = Some(
        db_word
            .definition
            .unwrap_or_default()
            .split("\n")
            .map(|x| x.to_owned())
            .collect::<Vec<String>>(),
    );

    word.translation = Some(
        db_word
            .translation
            .unwrap_or_default()
            .split("\n")
            .map(|x| x.to_owned())
            .collect::<Vec<String>>(),
    );

    word.tags = Some(
        db_word
            .tag
            .unwrap_or_default()
            .split_whitespace()
            .map(|x| FIELD_MAP_STR.get(x).expect("filed not found").to_string())
            .collect::<Vec<String>>()
            .join(", "),
    );
    word.exchanges = Some(
        db_word
            .exchange
            .unwrap_or_default()
            .split('/')
            .filter(|&x| x.chars().nth(0) != Some('1'))
            .map(|x| {
                let item = x.split(':').collect::<Vec<&str>>();
                let first_char = item.get(0).map(|s| s.chars().nth(0)).flatten();

                first_char.and_then(|c| {
                    FIELD_MAP_CHAR.get(&c).map(|v| {
                        let mut item = item.join(": ");
                        item.replace_range(0..1, v);
                        item
                    })
                })
            })
            .filter_map(|item| item)
            .collect::<Vec<String>>(),
    );
    word
}

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

#[tokio::test]
async fn test_query_word() {
    let qword = "ard";
    let word = query_word(qword).await.expect("query_word failed");

    match word {
        Some(word) => {
            eprintln!("{:#?}", word);
        }
        None => {
            eprintln!("word not found");
        }
    }
}

#[tokio::test]
async fn test_query_word_not_found() {
    let qword = "rusttttttttt";
    let word = query_word(qword).await.expect("query_word failed");

    match word {
        Some(word) => {
            eprintln!("{:#?}", word);
        }
        None => {
            eprintln!("word not found");
        }
    }
}
