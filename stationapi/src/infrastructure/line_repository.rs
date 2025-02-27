use async_trait::async_trait;
use sqlx::{MySql, MySqlConnection, Pool};
use std::sync::Arc;

use crate::domain::{
    entity::line::Line, error::DomainError, repository::line_repository::LineRepository,
};

#[derive(sqlx::FromRow, Clone)]
pub struct LineRow {
    pub line_cd: u32,
    pub company_cd: u32,
    pub line_type: Option<u32>,
    pub line_name: Option<String>,
    pub line_name_k: Option<String>,
    pub line_name_h: Option<String>,
    pub line_name_r: Option<String>,
    pub line_name_zh: Option<String>,
    pub line_name_ko: Option<String>,
    pub line_color_c: Option<String>,
    pub line_symbol_primary: Option<String>,
    pub line_symbol_secondary: Option<String>,
    pub line_symbol_extra: Option<String>,
    pub line_symbol_primary_color: Option<String>,
    pub line_symbol_secondary_color: Option<String>,
    pub line_symbol_extra_color: Option<String>,
    pub line_symbol_primary_shape: Option<String>,
    pub line_symbol_secondary_shape: Option<String>,
    pub line_symbol_extra_shape: Option<String>,
    pub e_status: u32,
    pub e_sort: u32,
    pub average_distance: f64,
    pub line_group_cd: Option<u32>,
    pub station_cd: Option<u32>,
    pub station_g_cd: Option<u32>,
}

impl From<LineRow> for Line {
    fn from(row: LineRow) -> Self {
        Self {
            line_cd: row.line_cd,
            company_cd: row.company_cd,
            company: None,
            line_name: row.line_name.unwrap_or_default(),
            line_name_k: row.line_name_k.unwrap_or_default(),
            line_name_h: row.line_name_h.unwrap_or_default(),
            line_name_r: row.line_name_r,
            line_name_zh: row.line_name_zh,
            line_name_ko: row.line_name_ko,
            line_color_c: row.line_color_c,
            line_type: row.line_type,
            line_symbols: vec![],
            line_symbol_primary: row.line_symbol_primary,
            line_symbol_secondary: row.line_symbol_secondary,
            line_symbol_extra: row.line_symbol_extra,
            line_symbol_primary_color: row.line_symbol_primary_color,
            line_symbol_secondary_color: row.line_symbol_secondary_color,
            line_symbol_extra_color: row.line_symbol_extra_color,
            line_symbol_primary_shape: row.line_symbol_primary_shape,
            line_symbol_secondary_shape: row.line_symbol_secondary_shape,
            line_symbol_extra_shape: row.line_symbol_extra_shape,
            e_status: row.e_status,
            e_sort: row.e_sort,
            station: None,
            train_type: None,
            line_group_cd: row.line_group_cd,
            station_cd: row.station_cd,
            station_g_cd: row.station_g_cd,
            average_distance: row.average_distance,
        }
    }
}

pub struct MyLineRepository {
    pool: Arc<Pool<MySql>>,
}

impl MyLineRepository {
    pub fn new(pool: Arc<Pool<MySql>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LineRepository for MyLineRepository {
    async fn find_by_id(&self, id: u32) -> Result<Option<Line>, DomainError> {
        let mut conn = self.pool.acquire().await?;
        InternalLineRepository::find_by_id(id, &mut conn).await
    }
    async fn find_by_station_id(&self, station_id: u32) -> Result<Option<Line>, DomainError> {
        let mut conn = self.pool.acquire().await?;
        InternalLineRepository::find_by_station_id(station_id, &mut conn).await
    }
    async fn get_by_ids(&self, ids: &[u32]) -> Result<Vec<Line>, DomainError> {
        let mut conn = self.pool.acquire().await?;
        InternalLineRepository::get_by_ids(ids, &mut conn).await
    }
    async fn get_by_station_group_id(&self, id: u32) -> Result<Vec<Line>, DomainError> {
        let mut conn = self.pool.acquire().await?;
        InternalLineRepository::get_by_station_group_id(id, &mut conn).await
    }
    async fn get_by_station_group_id_vec(
        &self,
        station_group_id_vec: &[u32],
    ) -> Result<Vec<Line>, DomainError> {
        let mut conn = self.pool.acquire().await?;
        InternalLineRepository::get_by_station_group_id_vec(station_group_id_vec, &mut conn).await
    }
    async fn get_by_line_group_id(&self, line_group_id: u32) -> Result<Vec<Line>, DomainError> {
        let mut conn = self.pool.acquire().await?;
        InternalLineRepository::get_by_line_group_id(line_group_id, &mut conn).await
    }
    async fn get_by_line_group_id_vec(
        &self,
        line_group_id_vec: &[u32],
    ) -> Result<Vec<Line>, DomainError> {
        let mut conn = self.pool.acquire().await?;
        InternalLineRepository::get_by_line_group_id_vec(line_group_id_vec, &mut conn).await
    }
    async fn get_by_line_group_id_vec_for_routes(
        &self,
        line_group_id_vec: &[u32],
    ) -> Result<Vec<Line>, DomainError> {
        let mut conn = self.pool.acquire().await?;
        InternalLineRepository::get_by_line_group_id_vec_for_routes(line_group_id_vec, &mut conn)
            .await
    }
    async fn get_by_name(
        &self,
        line_name: String,
        limit: Option<u32>,
    ) -> Result<Vec<Line>, DomainError> {
        let mut conn = self.pool.acquire().await?;
        InternalLineRepository::get_by_name(line_name, limit, &mut conn).await
    }
}

pub struct InternalLineRepository {}

impl InternalLineRepository {
    async fn find_by_id(id: u32, conn: &mut MySqlConnection) -> Result<Option<Line>, DomainError> {
        let rows: Option<LineRow> = sqlx::query_as!(
            LineRow,
            "SELECT *,
            CAST(NULL AS UNSIGNED INT) AS line_group_cd,
            CAST(NULL AS UNSIGNED INT) AS station_cd,
            CAST(NULL AS UNSIGNED INT) AS station_g_cd
            FROM `lines` AS l
            WHERE l.line_cd = ?
            AND l.e_status = 0",
            id
        )
        .fetch_optional(conn)
        .await?;
        let line: Option<Line> = rows.map(|row| row.into());

        let Some(line) = line else {
            return Ok(None);
        };

        Ok(Some(line))
    }

    async fn find_by_station_id(
        station_id: u32,
        conn: &mut MySqlConnection,
    ) -> Result<Option<Line>, DomainError> {
        let rows: Option<LineRow> = sqlx::query_as!(
            LineRow,
            "SELECT DISTINCT l.line_cd,
            l.company_cd,
            l.line_type,
            l.line_symbol_primary,
            l.line_symbol_secondary,
            l.line_symbol_extra,
            l.line_symbol_primary_color,
            l.line_symbol_secondary_color,
            l.line_symbol_extra_color,
            l.line_symbol_primary_shape,
            l.line_symbol_secondary_shape,
            l.line_symbol_extra_shape,
            l.e_status,
            l.e_sort,
            l.average_distance,
            s.station_cd,
            s.station_g_cd,
            sst.line_group_cd,
            COALESCE(a.line_name, l.line_name) AS line_name,
            COALESCE(a.line_name_k, l.line_name_k) AS line_name_k,
            COALESCE(a.line_name_h, l.line_name_h) AS line_name_h,
            COALESCE(a.line_name_r, l.line_name_r) AS line_name_r,
            COALESCE(a.line_name_zh, l.line_name_zh) AS line_name_zh,
            COALESCE(a.line_name_ko, l.line_name_ko) AS line_name_ko,
            COALESCE(a.line_color_c, l.line_color_c) AS line_color_c
        FROM `lines` AS l
            JOIN `stations` AS s ON s.station_cd = ?
            JOIN `station_station_types` AS sst ON sst.station_cd = s.station_cd AND sst.pass <> 1
            LEFT JOIN `line_aliases` AS la ON la.station_cd = s.station_cd
            LEFT JOIN `aliases` AS a ON la.alias_cd = a.id
        WHERE l.line_cd = s.line_cd",
            station_id,
        )
        .fetch_optional(conn)
        .await?;
        let line: Option<Line> = rows.map(|row| row.into());

        let Some(line) = line else {
            return Ok(None);
        };

        Ok(Some(line))
    }

    async fn get_by_ids(ids: &[u32], conn: &mut MySqlConnection) -> Result<Vec<Line>, DomainError> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        let params = format!("?{}", ", ?".repeat(ids.len() - 1));
        let query_str = format!(
            "SELECT * FROM `lines` WHERE line_cd IN ( {} ) AND e_status = 0",
            params
        );

        let mut query = sqlx::query_as::<_, LineRow>(&query_str);
        for id in ids {
            query = query.bind(id);
        }

        let rows = query.fetch_all(conn).await?;
        let lines: Vec<Line> = rows.into_iter().map(|row| row.into()).collect();

        Ok(lines)
    }

    async fn get_by_station_group_id(
        station_group_id: u32,
        conn: &mut MySqlConnection,
    ) -> Result<Vec<Line>, DomainError> {
        let rows: Vec<LineRow> = sqlx::query_as!(
            LineRow,
            "SELECT DISTINCT l.line_cd,
            l.company_cd,
            l.line_type,
            l.line_symbol_primary,
            l.line_symbol_secondary,
            l.line_symbol_extra,
            l.line_symbol_primary_color,
            l.line_symbol_secondary_color,
            l.line_symbol_extra_color,
            l.line_symbol_primary_shape,
            l.line_symbol_secondary_shape,
            l.line_symbol_extra_shape,
            l.e_status,
            l.e_sort,
            l.average_distance,
            COALESCE(a.line_name, l.line_name) AS line_name,
            COALESCE(a.line_name_k, l.line_name_k) AS line_name_k,
            COALESCE(a.line_name_h, l.line_name_h) AS line_name_h,
            COALESCE(a.line_name_r, l.line_name_r) AS line_name_r,
            COALESCE(a.line_name_zh, l.line_name_zh) AS line_name_zh,
            COALESCE(a.line_name_ko, l.line_name_ko) AS line_name_ko,
            COALESCE(a.line_color_c, l.line_color_c) AS line_color_c,
            sst.line_group_cd,
            s.station_cd,
            s.station_g_cd
        FROM `lines` AS l
        JOIN `stations` AS s ON s.station_g_cd = ?
            AND s.e_status = 0
        JOIN `station_station_types` AS sst ON sst.station_cd = s.station_cd AND sst.pass <> 1
        LEFT JOIN `line_aliases` AS la ON la.station_cd = s.station_cd
        LEFT JOIN `aliases` AS a ON la.alias_cd = a.id
        WHERE l.line_cd = s.line_cd
            AND l.e_status = 0",
            station_group_id
        )
        .fetch_all(conn)
        .await?;
        let lines: Vec<Line> = rows.into_iter().map(|row| row.into()).collect();

        Ok(lines)
    }

    async fn get_by_station_group_id_vec(
        station_group_id_vec: &[u32],
        conn: &mut MySqlConnection,
    ) -> Result<Vec<Line>, DomainError> {
        if station_group_id_vec.is_empty() {
            return Ok(vec![]);
        }

        let params = format!("?{}", ", ?".repeat(station_group_id_vec.len() - 1));
        let query_str = format!(
            "SELECT l.*,
            s.station_cd,
            s.station_g_cd,
            sst.line_group_cd,
            COALESCE(a.line_name, l.line_name) AS line_name,
            COALESCE(a.line_name_k, l.line_name_k) AS line_name_k,
            COALESCE(a.line_name_h, l.line_name_h) AS line_name_h,
            COALESCE(a.line_name_r, l.line_name_r) AS line_name_r,
            COALESCE(a.line_name_zh, l.line_name_zh) AS line_name_zh,
            COALESCE(a.line_name_ko, l.line_name_ko) AS line_name_ko,
            COALESCE(a.line_color_c, l.line_color_c) AS line_color_c
        FROM `lines` AS l
            JOIN `stations` AS s ON s.station_g_cd IN ( {} )
            AND s.e_status = 0
            LEFT JOIN `station_station_types` AS sst ON sst.station_cd = s.station_cd
            LEFT JOIN `line_aliases` AS la ON la.station_cd = s.station_cd
            LEFT JOIN `aliases` AS a ON la.alias_cd = a.id
        WHERE l.line_cd = s.line_cd
            AND l.e_status = 0
            AND IF(sst.line_group_cd IS NOT NULL, sst.pass <> 1, 1)
        GROUP BY s.station_cd",
            params
        );

        let mut query = sqlx::query_as::<_, LineRow>(&query_str);
        for id in station_group_id_vec {
            query = query.bind(id);
        }

        let rows = query.fetch_all(conn).await?;
        let lines: Vec<Line> = rows.into_iter().map(|row| row.into()).collect();

        Ok(lines)
    }

    async fn get_by_line_group_id(
        line_group_id: u32,
        conn: &mut MySqlConnection,
    ) -> Result<Vec<Line>, DomainError> {
        let rows: Vec<LineRow> = sqlx::query_as!(
            LineRow,
            "SELECT DISTINCT l.line_cd,
            l.company_cd,
            l.line_type,
            l.line_symbol_primary,
            l.line_symbol_secondary,
            l.line_symbol_extra,
            l.line_symbol_primary_color,
            l.line_symbol_secondary_color,
            l.line_symbol_extra_color,
            l.line_symbol_primary_shape,
            l.line_symbol_secondary_shape,
            l.line_symbol_extra_shape,
            l.e_status,
            l.e_sort,
            l.average_distance,
            s.station_cd,
            s.station_g_cd,
            sst.line_group_cd,
            COALESCE(a.line_name, l.line_name) AS line_name,
            COALESCE(a.line_name_k, l.line_name_k) AS line_name_k,
            COALESCE(a.line_name_h, l.line_name_h) AS line_name_h,
            COALESCE(a.line_name_r, l.line_name_r) AS line_name_r,
            COALESCE(a.line_name_zh, l.line_name_zh) AS line_name_zh,
            COALESCE(a.line_name_ko, l.line_name_ko) AS line_name_ko,
            COALESCE(a.line_color_c, l.line_color_c) AS line_color_c
        FROM `lines` AS l
            JOIN `station_station_types` AS sst ON sst.line_group_cd = ? AND sst.pass <> 1
            JOIN `stations` AS s ON s.station_cd = sst.station_cd
            AND s.e_status = 0
            LEFT JOIN `line_aliases` AS la ON la.station_cd = s.station_cd
            LEFT JOIN `aliases` AS a ON la.alias_cd = a.id
        WHERE l.line_cd = s.line_cd
            AND l.e_status = 0
            GROUP BY l.line_cd",
            line_group_id
        )
        .fetch_all(conn)
        .await?;
        let lines: Vec<Line> = rows.into_iter().map(|row| row.into()).collect();
        Ok(lines)
    }

    async fn get_by_line_group_id_vec(
        line_group_id_vec: &[u32],
        conn: &mut MySqlConnection,
    ) -> Result<Vec<Line>, DomainError> {
        if line_group_id_vec.is_empty() {
            return Ok(vec![]);
        }

        let params = format!("?{}", ", ?".repeat(line_group_id_vec.len() - 1));
        let query_str = format!(
            "SELECT DISTINCT
                l.*,
                sst.line_group_cd,
                COALESCE(a.line_name, l.line_name) AS line_name,
                COALESCE(a.line_name_k, l.line_name_k) AS line_name_k,
                COALESCE(a.line_name_h, l.line_name_h) AS line_name_h,
                COALESCE(a.line_name_r, l.line_name_r) AS line_name_r,
                COALESCE(a.line_name_zh, l.line_name_zh) AS line_name_zh,
                COALESCE(a.line_name_ko, l.line_name_ko) AS line_name_ko,
                COALESCE(a.line_color_c, l.line_color_c) AS line_color_c,
                s.station_cd,
                s.station_g_cd
            FROM `lines` AS l
            JOIN `station_station_types` AS sst ON sst.line_group_cd IN ( {} ) AND sst.pass <> 1
            JOIN `stations` AS s ON s.station_cd = sst.station_cd AND s.e_status = 0
            LEFT JOIN `line_aliases` AS la ON la.station_cd = s.station_cd
            LEFT JOIN `aliases` AS a ON la.alias_cd = a.id
            WHERE
                l.line_cd = s.line_cd
                AND l.e_status = 0
            GROUP BY sst.line_group_cd, l.line_cd",
            params
        );

        let mut query = sqlx::query_as::<_, LineRow>(&query_str);
        for id in line_group_id_vec {
            query = query.bind(id);
        }

        let rows = query.fetch_all(conn).await?;
        let lines: Vec<Line> = rows.into_iter().map(|row| row.into()).collect();

        Ok(lines)
    }
    async fn get_by_line_group_id_vec_for_routes(
        line_group_id_vec: &[u32],
        conn: &mut MySqlConnection,
    ) -> Result<Vec<Line>, DomainError> {
        if line_group_id_vec.is_empty() {
            return Ok(vec![]);
        }

        let params = format!("?{}", ", ?".repeat(line_group_id_vec.len() - 1));
        let query_str = format!(
            "SELECT
                l.*,
                sst.line_group_cd,
                COALESCE(a.line_name, l.line_name) AS line_name,
                COALESCE(a.line_name_k, l.line_name_k) AS line_name_k,
                COALESCE(a.line_name_h, l.line_name_h) AS line_name_h,
                COALESCE(a.line_name_r, l.line_name_r) AS line_name_r,
                COALESCE(a.line_name_zh, l.line_name_zh) AS line_name_zh,
                COALESCE(a.line_name_ko, l.line_name_ko) AS line_name_ko,
                COALESCE(a.line_color_c, l.line_color_c) AS line_color_c,
                s.station_cd,
                s.station_g_cd
            FROM `lines` AS l
            JOIN `station_station_types` AS sst ON sst.line_group_cd IN ( {} ) AND sst.pass <> 1
            JOIN `stations` AS s ON s.station_cd = sst.station_cd AND s.e_status = 0 AND s.line_cd = l.line_cd
            LEFT JOIN `line_aliases` AS la ON la.station_cd = s.station_cd
            LEFT JOIN `aliases` AS a ON la.alias_cd = a.id
            WHERE l.e_status = 0
            GROUP BY l.line_cd",
            params
        );

        let mut query = sqlx::query_as::<_, LineRow>(&query_str);
        for id in line_group_id_vec {
            query = query.bind(id);
        }

        let rows = query.fetch_all(conn).await?;
        let lines: Vec<Line> = rows.into_iter().map(|row| row.into()).collect();

        Ok(lines)
    }

    async fn get_by_name(
        line_name: String,
        limit: Option<u32>,
        conn: &mut MySqlConnection,
    ) -> Result<Vec<Line>, DomainError> {
        let line_name = format!("%{}%", line_name);

        let rows: Vec<LineRow> = sqlx::query_as!(
            LineRow,
            "SELECT *,
            CAST(NULL AS UNSIGNED INT) AS line_group_cd,
            CAST(NULL AS UNSIGNED INT) AS station_cd,
            CAST(NULL AS UNSIGNED INT) AS station_g_cd
            FROM `lines` AS l
            WHERE (
                    l.line_name LIKE ?
                    OR l.line_name_r LIKE ?
                    OR l.line_name_k LIKE ?
                    OR l.line_name_zh LIKE ?
                    OR l.line_name_ko LIKE ?
                )
                AND l.e_status = 0
            LIMIT ?",
            &line_name,
            &line_name,
            &line_name,
            &line_name,
            &line_name,
            limit.unwrap_or(1)
        )
        .fetch_all(conn)
        .await?;

        let lines: Vec<Line> = rows.into_iter().map(|row| row.into()).collect();

        Ok(lines)
    }
}
