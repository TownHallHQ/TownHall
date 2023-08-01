use std::ops::Deref;

/// Represents the result from a database query and a subset of records from
/// the same query.
#[derive(Debug, Default, Clone)]
pub struct QuerySet<T: Clone> {
    records: Vec<T>,
    count: u64,
}

impl<T: Clone> Deref for QuerySet<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.records
    }
}

impl<T: Clone> QuerySet<T> {
    /// Creates a new instance of `QuerySet` with it's records and `count`.
    ///
    /// The `count` value is used to determine the amount of records found
    /// in the query, that may or may not be the total amount of records in
    /// this `QuerySet` instance.
    pub fn new(records: Vec<T>, count: u64) -> Self {
        Self { records, count }
    }

    /// Subset of records matched by the database query and also included as
    /// part of the pagination process
    #[inline]
    pub fn records(&self) -> Vec<T> {
        self.records.to_owned()
    }

    /// Total amount of records in the database matched by the query
    #[inline]
    pub fn count(&self) -> u64 {
        self.count.to_owned()
    }

    /// Creates an empty `QuerySet`. This is useful for results where `T`
    /// doesn't implement `Default`.
    pub fn empty() -> Self {
        Self {
            records: Vec::new(),
            count: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Maps records stored internally
    pub fn inner_map<B, F>(self, f: F) -> QuerySet<B>
    where
        B: Clone,
        F: FnMut(T) -> B,
    {
        let records = self.records.into_iter().map(f).collect::<Vec<B>>();

        QuerySet {
            records,
            count: self.count,
        }
    }
}
