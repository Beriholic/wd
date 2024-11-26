use crate::repo::get_db_connection;
use crate::repo::model::{Stardict, Word};
use color_eyre::Result;
use sqlx::query_as;

pub async fn query_word(name: &str) -> Result<Option<Word>> {
    let db = get_db_connection().await?;

    let db_word: Stardict = query_as(
        "SELECT * FROM stardict WHERE word == $1",
    ).bind(name).fetch_one(&db).await?;

    db.close().await;
    Ok(Some(db_word.into()))
}

#[cfg(test)]
mod tests {
    use crate::repo::query::query_word;

    #[tokio::test]
    async fn test_query_word() {
        let result = query_word("rust").await;
        let word = result.unwrap_or(None);

        assert_eq!(word.is_some(), true);
        assert_eq!(word.unwrap().name, "rust");
    }
}

