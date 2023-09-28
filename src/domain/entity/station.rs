use crate::pb::StopCondition;

use super::{line::Line, station_number::StationNumber, train_type::TrainType};

#[derive(Clone, Debug)]
pub struct Station {
    pub station_cd: u32,
    pub station_g_cd: u32,
    pub station_name: String,
    pub station_name_k: String,
    pub station_name_r: Option<String>,
    pub station_name_zh: Option<String>,
    pub station_name_ko: Option<String>,
    pub station_numbers: Vec<StationNumber>,
    pub primary_station_number: Option<String>,
    pub secondary_station_number: Option<String>,
    pub extra_station_number: Option<String>,
    pub three_letter_code: Option<String>,
    pub line_cd: u32,
    pub line: Option<Box<Line>>,
    pub lines: Vec<Line>,
    pub pref_cd: u32,
    pub post: String,
    pub address: String,
    pub lon: f64,
    pub lat: f64,
    pub open_ymd: String,
    pub close_ymd: String,
    pub e_status: u32,
    pub e_sort: u32,
    pub stop_condition: StopCondition,
    pub distance: Option<f64>,
    pub station_types_count: i64,
    pub train_type: Option<TrainType>,
    // linesからJOIN
    pub company_cd: u32,
    pub line_name: String,
    pub line_name_k: String,
    pub line_name_h: String,
    pub line_name_r: Option<String>,
    pub line_name_zh: Option<String>,
    pub line_name_ko: Option<String>,
    pub line_color_c: String,
    pub line_type: u32,
    pub line_symbol_primary: Option<String>,
    pub line_symbol_secondary: Option<String>,
    pub line_symbol_extra: Option<String>,
    pub line_symbol_primary_color: Option<String>,
    pub line_symbol_secondary_color: Option<String>,
    pub line_symbol_extra_color: Option<String>,
    pub line_symbol_primary_shape: Option<String>,
    pub line_symbol_secondary_shape: Option<String>,
    pub line_symbol_extra_shape: Option<String>,
}

impl Station {
    #![allow(clippy::too_many_arguments)]
    pub fn new(
        station_cd: u32,
        station_g_cd: u32,
        station_name: String,
        station_name_k: String,
        station_name_r: Option<String>,
        station_name_zh: Option<String>,
        station_name_ko: Option<String>,
        station_numbers: Vec<StationNumber>,
        primary_station_number: Option<String>,
        secondary_station_number: Option<String>,
        extra_station_number: Option<String>,
        three_letter_code: Option<String>,
        line_cd: u32,
        line: Option<Box<Line>>,
        lines: Vec<Line>,
        pref_cd: u32,
        post: String,
        address: String,
        lon: f64,
        lat: f64,
        open_ymd: String,
        close_ymd: String,
        e_status: u32,
        e_sort: u32,
        stop_condition: StopCondition,
        distance: Option<f64>,
        station_types_count: i64,
        train_type: Option<TrainType>,
        company_cd: u32,
        line_name: String,
        line_name_k: String,
        line_name_h: String,
        line_name_r: Option<String>,
        line_name_zh: Option<String>,
        line_name_ko: Option<String>,
        line_color_c: String,
        line_type: u32,
        line_symbol_primary: Option<String>,
        line_symbol_secondary: Option<String>,
        line_symbol_extra: Option<String>,
        line_symbol_primary_color: Option<String>,
        line_symbol_secondary_color: Option<String>,
        line_symbol_extra_color: Option<String>,
        line_symbol_primary_shape: Option<String>,
        line_symbol_secondary_shape: Option<String>,
        line_symbol_extra_shape: Option<String>,
    ) -> Self {
        Self {
            station_cd,
            station_g_cd,
            station_name,
            station_name_k,
            station_name_r,
            station_name_zh,
            station_name_ko,
            station_numbers,
            primary_station_number,
            secondary_station_number,
            extra_station_number,
            three_letter_code,
            line_cd,
            line,
            lines,
            pref_cd,
            post,
            address,
            lon,
            lat,
            open_ymd,
            close_ymd,
            e_status,
            e_sort,
            stop_condition,
            distance,
            station_types_count,
            train_type,
            company_cd,
            line_name,
            line_name_k,
            line_name_h,
            line_name_r,
            line_name_zh,
            line_name_ko,
            line_color_c,
            line_type,
            line_symbol_primary,
            line_symbol_secondary,
            line_symbol_extra,
            line_symbol_primary_color,
            line_symbol_secondary_color,
            line_symbol_extra_color,
            line_symbol_primary_shape,
            line_symbol_secondary_shape,
            line_symbol_extra_shape,
        }
    }
}
