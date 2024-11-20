pub struct Stardict {
    pub id: i32,
    pub word: String,
    pub sw: String,
    pub phonetic: String,
    pub definition: String,
    pub translation: String,
    pub pos: String,
    pub collins: i32,
    pub oxford: i32,
    pub tag: String,
    pub bnc: i32,
    pub frq: i32,
    pub exchange: String,
    pub detail: String,
    pub audio: String,
}
impl Stardict {
    pub fn query() -> Option<Word> {
        let word = Self {
            id: 2622929,
            word: "rust".to_string(),
            sw: "rust".to_string(),
            phonetic: "rʌst".to_string(),
            definition: "n. a red or brown oxide coating on iron or steel caused by the action of oxygen and moisture\nn. a plant disease that produces a reddish-brown discoloration of leaves and stems; caused by various rust fungi\nn. the formation of reddish-brown ferric oxides on iron by low-temperature oxidation in the presence of water\nn. any of various fungi causing rust disease in plants".to_string(),
            translation: "n. 锈, 生锈, 衰退\nvi. 生锈, 衰退\nvt. 使生锈, 腐蚀".to_string(),
            pos: "v:40/n:60".to_string(),
            collins: 0,
            oxford: 0,
            tag: "cet4 cet6 ky toefl ielts".to_string(),
            bnc: 0,
            frq: 0,
            exchange: "i:rusting/d:rusted/3:rusts/s:rusts/p:rusted".to_string(),
            detail: "".to_string(),
            audio: "".to_string(),
        }.into();

        Some(word)
    }
    pub fn not_found() -> Word {
        Self {
            id: 0,
            word: "".to_string(),
            sw: "".to_string(),
            phonetic: "".to_string(),
            definition: "".to_string(),
            translation: "".to_string(),
            pos: "".to_string(),
            collins: 0,
            oxford: 0,
            tag: "".to_string(),
            bnc: 0,
            frq: 0,
            exchange: "".to_string(),
            detail: "".to_string(),
            audio: "".to_string(),
        }.into()
    }
}


pub struct Word {
    pub name: String,
    pub translation: String,
    pub definition: String,
    pub tags: String,
    pub exchange: String,
}
impl From<Stardict> for Word {
    fn from(value: Stardict) -> Self {
        Self {
            name: value.word,
            translation: value.translation,
            definition: value.definition,
            tags: parse_tag(&value.tag),
            exchange: parse_exchange(&value.exchange),
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
    use crate::repo::model::{parse_exchange, parse_tag, Stardict};

    #[test]
    fn test_not_found() {
        let word = Stardict::not_found();
        assert_eq!(word.name, "");
        assert_eq!(word.translation, "");
        assert_eq!(word.definition, "");
        assert_eq!(word.tags, "");
        assert_eq!(word.exchange, "");
    }

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
