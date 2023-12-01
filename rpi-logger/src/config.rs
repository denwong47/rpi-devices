use lazy_static::lazy_static;

lazy_static!(
    /// The default datetime format for use in this app.
    ///
    /// This shortened format is compatible with Python's `datetime.fromisoformat`.
    pub static ref DATETIME_FORMAT: &'static [time::format_description::FormatItem<'static>] =
        time::macros::format_description!("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:6]")
    ;
);

time::serde::format_description!(
    _serde_offset_date_time,
    OffsetDateTime,
    "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:6]"
);

/// Serialize and Deserialize module for `DATETIME_FORMAT`.
///
/// # Example
///
/// ```rust
/// use serde::{Serialize, Deserialize};
/// use time::OffsetDateTime;
/// use battleship::config::serde_offset_date_time;
///
/// #[derive(Serialize, Deserialize)]
/// pub struct MyStruct {
///     #[serde(with="serde_offset_date_time")]
///     timestamp: OffsetDateTime,
/// }
/// ```
pub mod serde_offset_date_time {
    #[allow(unused_imports)]
    pub use super::_serde_offset_date_time::*;
}
