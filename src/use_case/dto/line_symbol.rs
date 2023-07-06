use crate::{
    domain::entity::{line_symbol::LineSymbol, station::Station},
    pb::{LineSymbol as GrpcLineSymbol, StopCondition},
};

impl From<LineSymbol> for GrpcLineSymbol {
    fn from(symbol: LineSymbol) -> Self {
        Self {
            symbol: symbol.symbol,
            color: symbol.color,
            shape: symbol.shape,
        }
    }
}
