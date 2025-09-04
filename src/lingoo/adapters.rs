use pgvector::Vector;
use sqlx::query;

use crate::{
  lingoo::{
    models::{FindSimilarDocsError, FindSimilarDocsRequest, LingooRagDocument, StoreDocError},
    repository::LingooRepository,
  },
  postgres::Postgres,
};

use super::models::StoreDocRequest;

impl LingooRepository for Postgres {
  async fn find_similar_docs(
    &self,
    request: &FindSimilarDocsRequest,
  ) -> Result<Vec<LingooRagDocument>, FindSimilarDocsError> {
    let similar_docs = query!(
      "SELECT (1 - (embedding <=> $1)) AS distance, id, embedding as \"embedding: Vector\", content, created_at, updated_at FROM lingoo_rag ORDER BY distance ASC",
      Vector::from(request.query().clone().into_inner()) as Vector,
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

  async fn store_doc(&self, request: &StoreDocRequest) -> Result<(), StoreDocError> {
    query!(
      "INSERT INTO lingoo_rag (content, embedding) VALUES ($1, $2) RETURNING id",
      request.content(),
      Vector::from(request.embedding().clone().into_inner()) as Vector
    )
    .fetch_one(self.pool())
    .await
    .map_err(|_| StoreDocError::Unknown)?;

    Ok(())
  }
}
