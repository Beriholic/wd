use crate::repo::get_db_connection;
use crate::repo::model::{Stardict, Word};
use color_eyre::Result;
use sqlx::query_as;

pub async fn query_word(name: &str) -> Result<Word> {
    let db = get_db_connection().await?;

    let db_word = query_as!(
        Stardict,
        "SELECT * FROM stardict WHERE word == $1",
        name
    ).fetch_one(&db).await?;

    db.close().await;
    Ok(db_word.into())
}

#[cfg(test)]
mod tests {
    use crate::repo::model::Word;
    use crate::repo::query::query_word;

    #[tokio::test]
    async fn test_query_word() {
        let result = query_word("rust").await;
        let word = result.unwrap_or(Word::default());

        assert_eq!(word.name, "rust");
    }
}

