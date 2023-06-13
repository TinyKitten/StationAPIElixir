use anyhow::Result;

use super::station::Station;

#[async_trait::async_trait]
pub trait StationRepository {
    async fn find_by_id(&self, id: u32) -> Result<Station>;
    async fn find_by_group_id(&self, group_id: u32) -> Result<Vec<Station>>;
    async fn find_by_line_id(&self, line_id: u32) -> Result<Vec<Station>>;
    async fn find_by_name(&self, name: &str) -> Result<Vec<Station>>;
    async fn find_by_coordinates(
        &self,
        latitude: f64,
        longitude: f64,
        limit: Option<i32>,
    ) -> Result<Vec<Station>>;
}
