use pgvector::Vector;
use sqlx::query;

use crate::{
  entities::{common::AnyText, embedding::Embedding},
  lingoo::{
    models::{FindSimilarDocsError, LingooRagDocument, StoreDocError},
    repository::LingooRepository,
  },
  postgres::Postgres,
  rag::models::TopK,
};

impl LingooRepository for Postgres {
  async fn find_similar_docs(
    &self,
    query: Embedding,
    top_k: TopK,
  ) -> Result<Vec<LingooRagDocument>, FindSimilarDocsError> {
    let similar_docs = query!(
      "SELECT embedding <=> $1 AS distance, id, embedding AS \"embedding: Vector\", content, created_at, updated_at FROM lingoo_rag ORDER BY distance ASC LIMIT $2",
      Vector::from(query.into_inner()) as Vector,
      Into::<i64>::into(top_k.as_u8()),
    )
    .fetch_all(self.pool())
    .await
    .map_err(|_| FindSimilarDocsError::Unknown)?
    .into_iter()
    .map(|d| {
      LingooRagDocument::new(
        d.id.into(),
        d.embedding.to_vec().into(),
        d.content.into(),
        (d.created_at.unix_timestamp() as u64).into(),
        (d.updated_at.unix_timestamp() as u64).into()
      )
    }).collect();

    Ok(similar_docs)
  }

  async fn store_doc(&self, content: &AnyText, embedding: Embedding) -> Result<(), StoreDocError> {
    query!(
      "INSERT INTO lingoo_rag (content, embedding) VALUES ($1, $2) RETURNING id",
      content.as_ref(),
      Vector::from(embedding.into_inner()) as Vector
    )
    .fetch_one(self.pool())
    .await
    .map_err(|_| StoreDocError::Unknown)?;

    Ok(())
  }
}
