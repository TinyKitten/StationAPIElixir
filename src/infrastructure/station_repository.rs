use async_trait::async_trait;
use bigdecimal::{BigDecimal, ToPrimitive};
use sqlx::{MySql, MySqlConnection, Pool};

use crate::domain::{
    entity::{station::Station, train_type::TrainType},
    error::DomainError,
    repository::station_repository::StationRepository,
};

#[derive(sqlx::FromRow, Clone)]
struct StationRow {
    station_cd: u32,
    station_g_cd: u32,
    station_name: String,
    station_name_k: String,
    station_name_r: String,
    station_name_zh: String,
    station_name_ko: String,
    primary_station_number: Option<String>,
    secondary_station_number: Option<String>,
    extra_station_number: Option<String>,
    three_letter_code: Option<String>,
    line_cd: u32,
    pref_cd: u32,
    post: String,
    address: String,
    lon: BigDecimal,
    lat: BigDecimal,
    open_ymd: String,
    close_ymd: String,
    e_status: u32,
    e_sort: u32,
}

impl From<StationRow> for Station {
    fn from(row: StationRow) -> Self {
        Self {
            station_cd: row.station_cd,
            station_g_cd: row.station_g_cd,
            station_name: row.station_name,
            station_name_k: row.station_name_k,
            station_name_r: row.station_name_r,
            station_name_zh: row.station_name_zh,
            station_name_ko: row.station_name_ko,
            station_numbers: vec![],
            primary_station_number: row.primary_station_number,
            secondary_station_number: row.secondary_station_number,
            extra_station_number: row.extra_station_number,
            three_letter_code: row.three_letter_code,
            line_cd: row.line_cd,
            line: None,
            lines: vec![],
            pref_cd: row.pref_cd,
            post: row.post,
            address: row.address,
            lon: row
                .lon
                .to_f64()
                .expect("Failed to convert BigDecimal to f64"),
            lat: row
                .lat
                .to_f64()
                .expect("Failed to convert BigDecimal to f64"),
            open_ymd: row.open_ymd,
            close_ymd: row.close_ymd,
            e_status: row.e_status,
            e_sort: row.e_sort,
            train_types: vec![],
            pass: false,
            distance: None,
        }
    }
}

#[derive(sqlx::FromRow, Clone)]
struct StationWithDistanceRow {
    station_cd: u32,
    station_g_cd: u32,
    station_name: String,
    station_name_k: String,
    station_name_r: String,
    station_name_zh: String,
    station_name_ko: String,
    primary_station_number: Option<String>,
    secondary_station_number: Option<String>,
    extra_station_number: Option<String>,
    three_letter_code: Option<String>,
    line_cd: u32,
    pref_cd: u32,
    post: String,
    address: String,
    lon: BigDecimal,
    lat: BigDecimal,
    open_ymd: String,
    close_ymd: String,
    e_status: u32,
    e_sort: u32,
    train_types: Vec<TrainType>,
    pass: bool,
    distance: Option<f64>,
}

impl From<StationWithDistanceRow> for Station {
    fn from(row: StationWithDistanceRow) -> Self {
        Self {
            station_cd: row.station_cd,
            station_g_cd: row.station_g_cd,
            station_name: row.station_name,
            station_name_k: row.station_name_k,
            station_name_r: row.station_name_r,
            station_name_zh: row.station_name_zh,
            station_name_ko: row.station_name_ko,
            station_numbers: vec![],
            primary_station_number: row.primary_station_number,
            secondary_station_number: row.secondary_station_number,
            extra_station_number: row.extra_station_number,
            three_letter_code: row.three_letter_code,
            line_cd: row.line_cd,
            line: None,
            lines: vec![],
            pref_cd: row.pref_cd,
            post: row.post,
            address: row.address,
            lon: row
                .lon
                .to_f64()
                .expect("Failed to convert BigDecimal to f64"),
            lat: row
                .lat
                .to_f64()
                .expect("Failed to convert BigDecimal to f64"),
            open_ymd: row.open_ymd,
            close_ymd: row.close_ymd,
            e_status: row.e_status,
            e_sort: row.e_sort,
            train_types: row.train_types,
            pass: false,
            distance: row.distance,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MyStationRepository {
    pool: Pool<MySql>,
}

impl MyStationRepository {
    pub fn new(pool: Pool<MySql>) -> Self {
        Self { pool }
    }
}

const DEFAULT_COLUMN_COUNT: u32 = 1;

#[async_trait]
impl StationRepository for MyStationRepository {
    async fn find_by_id(&self, id: u32) -> Result<Option<Station>, DomainError> {
        let mut conn = self.pool.acquire().await?;
        InternalStationRepository::find_by_id(id, &mut conn).await
    }
    async fn get_by_line_id(&self, line_id: u32) -> Result<Vec<Station>, DomainError> {
        let mut conn = self.pool.acquire().await?;
        InternalStationRepository::get_by_line_id(line_id, &mut conn).await
    }
    async fn get_by_station_group_id(
        &self,
        station_group_id: u32,
    ) -> Result<Vec<Station>, DomainError> {
        let mut conn: sqlx::pool::PoolConnection<MySql> = self.pool.acquire().await?;
        InternalStationRepository::get_by_station_group_id(station_group_id, &mut conn).await
    }

    async fn get_stations_by_coordinates(
        &self,
        latitude: f64,
        longitude: f64,
        limit: Option<u32>,
    ) -> Result<Vec<Station>, DomainError> {
        let mut conn = self.pool.acquire().await?;
        InternalStationRepository::get_stations_by_coordinates(
            latitude, longitude, limit, &mut conn,
        )
        .await
    }

    async fn get_stations_by_name(
        &self,
        station_name: String,
        limit: Option<u32>,
    ) -> Result<Vec<Station>, DomainError> {
        let mut conn = self.pool.acquire().await?;
        InternalStationRepository::get_stations_by_name(station_name, limit, &mut conn).await
    }
}

struct InternalStationRepository {}

impl InternalStationRepository {
    async fn find_by_id(
        id: u32,
        conn: &mut MySqlConnection,
    ) -> Result<Option<Station>, DomainError> {
        let rows: Option<StationRow> =
            sqlx::query_as("SELECT * FROM stations WHERE station_cd = ? AND e_status = 0 ORDER BY e_sort, station_cd")
                .bind(id)
                .fetch_optional(conn)
                .await?;

        let station = rows.map(|row| row.into());

        Ok(station)
    }
    async fn get_by_line_id(
        line_id: u32,
        conn: &mut MySqlConnection,
    ) -> Result<Vec<Station>, DomainError> {
        let station_row: Vec<StationRow> = sqlx::query_as(
            "SELECT * FROM stations WHERE line_cd = ? AND e_status = 0 ORDER BY e_sort, station_cd",
        )
        .bind(line_id)
        .fetch_all(conn)
        .await?;

        let stations = station_row.into_iter().map(|row| row.into()).collect();

        Ok(stations)
    }

    async fn get_by_station_group_id(
        group_id: u32,
        conn: &mut MySqlConnection,
    ) -> Result<Vec<Station>, DomainError> {
        let rows: Vec<StationRow> =
            sqlx::query_as("SELECT * FROM stations WHERE station_g_cd = ? AND e_status = 0 ORDER BY e_sort, station_cd")
                .bind(group_id)
                .fetch_all(conn)
                .await?;

        let stations = rows.into_iter().map(|row| row.into()).collect();

        Ok(stations)
    }

    async fn get_stations_by_coordinates(
        latitude: f64,
        longitude: f64,
        limit: Option<u32>,
        conn: &mut MySqlConnection,
    ) -> Result<Vec<Station>, DomainError> {
        let query_str = "SELECT *,
        (
          6371 * acos(
          cos(radians(?))
          * cos(radians(lat))
          * cos(radians(lon) - radians(?))
          + sin(radians(?))
          * sin(radians(lat))
          )
        ) AS distance
        FROM
        stations as s1
        WHERE
        e_status = 0
        AND
        station_cd = (
          SELECT station_cd
          FROM stations as s2
          WHERE s1.station_g_cd = s2.station_g_cd
          LIMIT 1
        )
        ORDER BY distance
        LIMIT ?";

        let rows = sqlx::query_as::<_, StationRow>(query_str)
            .bind(latitude)
            .bind(longitude)
            .bind(latitude)
            .bind(limit.unwrap_or(DEFAULT_COLUMN_COUNT))
            .fetch_all(conn)
            .await?;

        let stations = rows.into_iter().map(|row| row.into()).collect();

        Ok(stations)
    }

    async fn get_stations_by_name(
        station_name: String,
        limit: Option<u32>,
        conn: &mut MySqlConnection,
    ) -> Result<Vec<Station>, DomainError> {
        let query_str: String = format!(
            "SELECT * FROM stations
            WHERE (
                station_name LIKE '%{}%'
                OR station_name_r LIKE '%{}%'
                OR station_name_k LIKE '%{}%'
                OR station_name_zh LIKE '%{}%'
                OR station_name_ko LIKE '%{}%'
        )
            AND e_status = 0
            ORDER BY e_sort, station_cd
            LIMIT {}
        ",
            station_name,
            station_name,
            station_name,
            station_name,
            station_name,
            limit.unwrap_or(DEFAULT_COLUMN_COUNT)
        );
        let result = sqlx::query_as::<_, StationRow>(&query_str)
            .fetch_all(conn)
            .await;
        match result {
            Ok(rows) => Ok(rows.into_iter().map(|row| row.into()).collect()),
            Err(err) => Err(err.into()),
        }
    }
}
