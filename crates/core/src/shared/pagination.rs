use std::fmt::Display;

use pxid::Pxid;
use sea_orm::{Cursor, FromQueryResult, SelectModel};
use thiserror::Error;

/// The default amount of items to paginate when no cursor fields are provided
const DEFAULT_FIRST_ITEMS_NO: u64 = 20;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PageInfo {
    pub has_next_pages: bool,
    pub has_prev_pages: bool,
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum PaginationError {
    #[error("Argument `{0}` cannot be combined with `{1}`")]
    NotCombinableArguments(CursorSelection, CursorStart),
    #[error("You must provide either one of `after` or `before` not both")]
    AmbiguousCursorStart,
    #[error("You must provide either one of `first` or `last` not both")]
    AmbiguousCursorSelection,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CursorStart {
    After(Pxid),
    Before(Pxid),
}

impl Display for CursorStart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_str = match self {
            CursorStart::After(_) => "after",
            CursorStart::Before(_) => "before",
        };

        write!(f, "{}", as_str)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CursorSelection {
    First(u64),
    Last(u64),
}

impl Display for CursorSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_str = match self {
            CursorSelection::First(_) => "first",
            CursorSelection::Last(_) => "last",
        };

        write!(f, "{}", as_str)
    }
}

impl Default for CursorSelection {
    fn default() -> Self {
        CursorSelection::First(DEFAULT_FIRST_ITEMS_NO)
    }
}

/// Cursor Pagination abstraction for SQL Queries
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Pagination {
    start: Option<CursorStart>,
    selection: CursorSelection,
}

impl Pagination {
    pub fn new(
        after: Option<Pxid>,
        before: Option<Pxid>,
        first: Option<usize>,
        last: Option<usize>,
    ) -> Result<Self, PaginationError> {
        if first.is_some() && before.is_some() {
            return Err(PaginationError::NotCombinableArguments(
                CursorSelection::First(0),
                CursorStart::Before(Pxid::default()),
            ));
        }

        if last.is_some() && after.is_some() {
            return Err(PaginationError::NotCombinableArguments(
                CursorSelection::Last(0),
                CursorStart::After(Pxid::default()),
            ));
        }

        let start: Option<CursorStart> = match (after, before) {
            (None, None) => None,
            (None, Some(before)) => Some(CursorStart::Before(before)),
            (Some(after), None) => Some(CursorStart::After(after)),
            (Some(_), Some(_)) => {
                return Err(PaginationError::AmbiguousCursorStart);
            }
        };

        let selection: CursorSelection = match (first, last) {
            (None, None) => CursorSelection::default(),
            (None, Some(last)) => CursorSelection::Last(last as u64),
            (Some(first), None) => CursorSelection::First(first as u64),
            (Some(_), Some(_)) => {
                return Err(PaginationError::AmbiguousCursorSelection);
            }
        };

        Ok(Self { start, selection })
    }

    pub fn apply<T: FromQueryResult>(&self, cursor: &mut Cursor<SelectModel<T>>) {
        match self.start() {
            Some(CursorStart::After(value)) => {
                cursor.after(value.to_string());
            }
            Some(CursorStart::Before(value)) => {
                cursor.before(value.to_string());
            }
            None => {}
        };

        match self.selection() {
            CursorSelection::First(value) => cursor.first(value),
            CursorSelection::Last(value) => cursor.last(value),
        };
    }

    /// Creates a cursor pagination instance fetching the first value
    pub fn first() -> Self {
        Self {
            start: None,
            selection: CursorSelection::First(1),
        }
    }

    pub fn start(&self) -> Option<CursorStart> {
        self.start
    }

    pub fn selection(&self) -> CursorSelection {
        self.selection
    }

    pub fn selection_count(&self) -> u64 {
        match self.selection {
            CursorSelection::First(val) => val,
            CursorSelection::Last(val) => val,
        }
    }

    pub fn get_page_info(&self, records: u64) -> PageInfo {
        let mut page_info = PageInfo::default();
        let records_left = records > self.selection_count();
        let has_other_pages = self.start.is_some();

        match self.selection {
            CursorSelection::First(_) => {
                page_info.has_next_pages = records_left;
                page_info.has_prev_pages = has_other_pages;
            }
            CursorSelection::Last(_) => {
                page_info.has_next_pages = has_other_pages;
                page_info.has_prev_pages = records_left;
            }
        };

        page_info
    }
}

#[cfg(test)]
mod tests {
    use pxid::Pxid;

    use super::{
        CursorSelection, CursorStart, Pagination, PaginationError, DEFAULT_FIRST_ITEMS_NO,
    };

    #[test]
    fn creates_default_cursor() {
        let cursor = Pagination::default();

        assert!(cursor.start.is_none());
        assert_eq!(
            cursor.selection,
            CursorSelection::First(DEFAULT_FIRST_ITEMS_NO)
        );
    }

    #[test]
    fn returns_error_on_both_start_params_present() {
        let cursor = Pagination::new(
            Some(Pxid::new_unchecked("test")),
            Some(Pxid::new_unchecked("test")),
            None,
            None,
        );

        assert!(cursor.is_err());
        assert_eq!(cursor, Err(PaginationError::AmbiguousCursorStart));
    }

    #[test]
    fn returns_error_on_both_selection_params_present() {
        let cursor = Pagination::new(None, None, Some(100), Some(100));

        assert!(cursor.is_err());
        assert_eq!(cursor, Err(PaginationError::AmbiguousCursorSelection));
    }

    #[test]
    fn complains_if_before_and_first_are_both_present() {
        let cursor = Pagination::new(None, Some(Pxid::default()), Some(1), None);

        assert!(cursor.is_err());
        assert_eq!(
            cursor,
            Err(PaginationError::NotCombinableArguments(
                CursorSelection::First(0),
                CursorStart::Before(Pxid::default())
            ))
        );
    }

    #[test]
    fn complains_if_after_and_last_are_both_present() {
        let cursor = Pagination::new(Some(Pxid::default()), None, None, Some(1));

        assert!(cursor.is_err());
        assert_eq!(
            cursor,
            Err(PaginationError::NotCombinableArguments(
                CursorSelection::Last(0),
                CursorStart::After(Pxid::default())
            ))
        );
    }

    #[test]
    fn creates_a_pagination_on_first() {
        let cursor = Pagination::first();

        assert_eq!(
            cursor,
            Pagination {
                start: None,
                selection: CursorSelection::First(1),
            }
        );
    }
}
