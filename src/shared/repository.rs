use std::fmt::Debug;

use bincode::{deserialize, serialize};
use serde::de::DeserializeOwned;
use serde::Serialize;

/// A record represents an entity stored into the tree
pub trait Record: Clone {
    /// Retrieves the Record's ID
    fn get_id(&self) -> &[u8];
    /// Sets the Record's ID
    fn set_id(&mut self, id: &[u8]);
    /// Set updated at
    fn set_updated_at(&mut self);
}

pub type UpdateRecordFn<R, Error> = Box<dyn FnOnce(R) -> Result<R, Error>>;

pub trait Repository<const TREE: char, T: DeserializeOwned + Record + Serialize + Send> {
    type Error: Debug;

    /// Retrieves a tree fromt the Sled instance
    fn get_tree(&self) -> sled::Result<sled::Tree>;

    /// Inserts a new Record into the tree by creating an instance of the
    /// record from a DTO
    fn create_with_key<K, U>(&self, key: K, dto: U) -> Result<T, Self::Error>
    where
        K: AsRef<[u8]>,
        U: Into<T> + Send + Serialize,
    {
        let tree = self.get_tree().unwrap();
        let mut record: T = dto.into();

        record.set_id(key.as_ref());
        let encoded = serialize(&record).unwrap();

        tree.insert(key, encoded).unwrap();

        Ok(record)
    }

    /// Fetches a record from the tree by its ID
    fn find_by_key<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<T>, Self::Error> {
        let tree = self.get_tree().unwrap();
        let maybe_encoded_record = tree.get(key).unwrap();

        if let Some(encoded_record) = maybe_encoded_record {
            let record: T = deserialize(&encoded_record).unwrap();

            return Ok(Some(record));
        }

        Ok(None)
    }

    /// Retrieves every record from the tree
    fn list(&self) -> Result<Vec<T>, Self::Error> {
        let tree = self.get_tree().unwrap();

        tree.iter()
            .map(|item| {
                let (_, encoded_record) = item.unwrap();
                Ok(bincode::deserialize::<T>(&encoded_record).unwrap())
            })
            .collect()
    }

    /// Updates a record in the Tree by providing the key.
    ///
    /// A `update_fn` must be provided which will return the next version for
    /// the entry.
    fn update<K: AsRef<[u8]>>(
        &self,
        key: K,
        update_fn: UpdateRecordFn<T, Self::Error>,
    ) -> Result<T, Self::Error> {
        let tree = self.get_tree().unwrap();

        if let Some(record) = self.find_by_key(&key).unwrap() {
            let mut next_record = update_fn(record.clone()).unwrap();
            next_record.set_updated_at();
            let encoded = serialize(&next_record).unwrap();

            tree.insert(key, encoded).unwrap();

            return Ok(next_record);
        }

        panic!("Failed")
    }
}
