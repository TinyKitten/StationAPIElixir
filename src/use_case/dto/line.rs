use crate::{domain::entity::line::Line, pb::Line as GrpcLine};

impl From<Line> for GrpcLine {
    fn from(line: Line) -> Self {
        Self {
            id: line.line_cd,
            name_short: line.line_name,
            name_katakana: line.line_name_k,
            name_full: line.line_name_h,
            name_roman: line.line_name_r,
            name_chinese: line.line_name_zh.unwrap_or("".to_string()),
            name_korean: line.line_name_ko.unwrap_or("".to_string()),
            color: line.line_color_c,
            line_type: line.line_type as i32,
            line_symbols: line.line_symbols.into_iter().map(|s| s.into()).collect(),
            status: line.e_status as i32,
            station: None,
            company: line.company,
        }
    }
}
