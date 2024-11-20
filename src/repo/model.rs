#[allow(dead_code)]
#[derive(Debug, sqlx::FromRow, Default)]
pub struct Stardict {
    pub id: i64,
    pub word: Option<String>,
    pub sw: Option<String>,
    pub phonetic: Option<String>,
    pub definition: Option<String>,
    pub translation: Option<String>,
    pub pos: Option<String>,
    pub collins: Option<i64>,
    pub oxford: Option<i64>,
    pub tag: Option<String>,
    pub bnc: Option<i64>,
    pub frq: Option<i64>,
    pub exchange: Option<String>,
    pub detail: Option<String>,
    pub audio: Option<String>,
}
// pub struct Stardict {
//     // pub id: i64,
//     pub word: Option<String>,
//     // pub sw: Option<String>,
//     pub phonetic: Option<String>,
//     pub definition: Option<String>,
//     pub translation: Option<String>,
//     // pub pos: Option<String>,
//     // pub collins: Option<i64>,
//     // pub oxford: Option<i64>,
//     pub tag: Option<String>,
//     // pub bnc: Option<i64>,
//     // pub frq: Option<i64>,
//     pub exchange: Option<String>,
//     // pub detail: Option<String>,
//     // pub audio: Option<String>,
// }

pub struct Word {
    pub name: String,
    pub translation: String,
    pub definition: String,
    pub tags: String,
    pub exchange: String,
}

impl Default for Word {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            translation: "".to_string(),
            definition: "".to_string(),
            tags: "".to_string(),
            exchange: "".to_string(),
        }
    }
}

impl From<Stardict> for Word {
    fn from(value: Stardict) -> Self {
        Self {
            name: format!(
                "{} [{}]",
                value.word.unwrap_or("".to_string()),
                value.phonetic.unwrap_or("".to_string())
            ),
            translation: value.translation.unwrap_or("".to_string()),
            definition: value.definition.unwrap_or("".to_string()),
            tags: parse_tag(&value.tag.unwrap_or("".to_string())),
            exchange: parse_exchange(&value.exchange.unwrap_or("".to_string())),
        }
    }
}

fn parse_tag(tag: &str) -> String {
    let parts: Vec<&str> = tag.split_whitespace().collect();
    let mut tags = Vec::new();

    for part in parts {
        tags.push(match part {
            "cet4" => "四级",
            "cet6" => "六级",
            "ky" => "考研",
            "toefl" => "托福",
            "ielts" => "雅思",
            "zk" => "中考",
            "gk" => "高考",
            _ => ""
        })
    }
    tags.join(" ")
}

fn parse_exchange(exchange: &str) -> String {
    let mut exc = Vec::new();

    for part in exchange.split('/') {
        let mut iter = part.split(':');
        if let (Some(tag), Some(word)) = (iter.next(), iter.next()) {
            exc.push(match tag {
                "p" => format!("过去式:{}", word),
                "d" => format!("过去分词:{}", word),
                "i" => format!("现在分词:{}", word),
                "3" => format!("第三人称单数:{}", word),
                "r" => format!("形容词比较级:{}", word),
                "t" => format!("形容词最高级:{}", word),
                "s" => format!("复数形式:{}", word),
                "0" => format!("Lemma:{}", word),
                "1" => format!("Lemma 的变换形式:{}", word),
                _ => continue,
            })
        }
    }

    exc.join("\n")
}


#[cfg(test)]
mod tests {
    use crate::repo::model::{parse_exchange, parse_tag};

    #[test]
    fn test_parse_tag() {
        let tag = parse_tag("cet4 cet6 ky toefl ielts");
        assert_eq!(tag, "四级 六级 考研 托福 雅思");
    }
    #[test]
    fn test_parse_exchange() {
        let exchange = parse_exchange("i:rusting/d:rusted/3:rusts/s:rusts/p:rusted");
        assert_eq!(exchange, "现在分词:rusting\n过去分词:rusted\n第三人称单数:rusts\n复数形式:rusts\n过去式:rusted");
    }
}
