//! # multidict
//!
//! The `multidict` crate was inspired by Python `MultiDict` library
//! `multidict` is useful for working with HTTP headers, URL query, form-data args etc
//! HTTP Headers and URL query string require specific data structure: `multidict`.
//! It behaves mostly like a regular map but it may have several values for the same key
//! and preserves insertion ordering
//!
//! ## Examples
//! ```
//! use multidict::MultiDict;
//!
//! let mut map = MultiDict::new();
//! let mut map = MultiDict::new();
//! map.add(["some_key".to_string(), "some_value".to_string()]);
//! println!("{map:?}"); // MultiDict { elements: [["some_key", "some_value"]] }
//! map.add(["some_key".to_string(), "some_value".to_string()]);
//! println!("{map:?}"); // MultiDict { elements: [["some_key", "some_value"], ["some_key", "some_value"]] }
//! ```
//!
#[derive(Debug, Clone)]
pub struct MultiDict {
    pub elements: Vec<[String; 2]>,
}
impl MultiDict {
    pub fn new() -> Self {
        MultiDict {
            elements: Vec::new(),
        }
    }

    /// Creates new object with preset capacity
    pub fn new_capacity(capacity: &usize) -> Self {
        MultiDict {
            elements: Vec::with_capacity(capacity.clone()),
        }
    }

    /// Return the number of items in MultiDict
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Append (key, value) pair to the MultiDict.
    ///
    /// # Examples
    /// ```
    /// use multidict::MultiDict;
    ///
    /// let mut map = MultiDict::new();
    /// map.add(["some_key".to_string(), "some_value".to_string()]);
    /// println!("{map:?}"); // MultiDict { elements: [["some_key", "some_value"]] }
    /// map.add(["some_key".to_string(), "some_value".to_string()]);
    /// println!("{map:?}"); // MultiDict { elements: [["some_key", "some_value"], ["some_key", "some_value"]] }
    /// ```
    pub fn add(&mut self, new_item: [String; 2]) {
        self.elements.push(new_item);
    }

    /// Return the first key-value pair for key if key is in the MultiDict
    pub fn get(&self, key: &str) -> Result<&[String; 2], &str> {
        for item in &self.elements {
            if item.get(0).unwrap().eq(key) {
                return Ok(item);
            }
        }
        Err("No matching key found for key")
    }

    /// If key is in the MultiDict, remove it and return its the first value, else return error
    pub fn popone(&mut self, key: &str) -> Result<[String; 2], &str> {
        for (idx, item) in self.elements.iter().enumerate() {
            if item.get(0).unwrap().eq(key) {
                return Ok(self.elements.remove(idx));
            }
        }
        Err("No matching key found for key")
    }

    /// Return a list of all key-values for key if key is in the MultiDict
    pub fn getall(&self, key: &str) -> Result<Vec<&[String; 2]>, &str> {
        let mut results = Vec::new();
        for item in &self.elements {
            if item.get(0).unwrap().eq(key) {
                results.push(item);
            }
        }
        if !results.is_empty() {
            Ok(results)
        } else {
            Err("No matching key found for key")
        }
    }

    /// Return True if MultiDict has a key, else False.
    ///
    /// # Examples
    ///
    /// If key exists
    /// ```
    /// use multidict::MultiDict;
    ///
    /// let mut map = MultiDict::new();
    /// map.add(["some_key".to_string(), "some_value_1".to_string()]);
    /// map.add(["some_key".to_string(), "some_value_2".to_string()]);
    /// println!("{:?}", map.contains("some_key")); // true
    /// ```
    ///
    /// If key not exists
    /// ```
    /// use multidict::MultiDict;
    ///
    /// let mut map = MultiDict::new();
    /// map.add(["some_key".to_string(), "some_value_1".to_string()]);
    /// map.add(["some_key".to_string(), "some_value_2".to_string()]);
    /// println!("{:?}", map.contains("some_other_key")); // false
    /// ```
    pub fn contains(&self, key: &str) -> bool {
        for item in &self.elements {
            if item.get(0).unwrap().eq(key) {
                return true;
            }
        }
        false
    }

    /// Return Vec of all keys form MultiDict.
    /// View contains all keys, possibly with duplicates.
    ///
    /// # Examples
    ///
    /// ```
    /// use multidict::MultiDict;
    ///
    /// let mut map = MultiDict::new();
    /// map.add(["some_key".to_string(), "some_value_1".to_string()]);
    /// map.add(["some_key".to_string(), "some_value_2".to_string()]);
    /// map.add(["some_other_key".to_string(), "some_value_2".to_string()]);
    /// println!("{:?}", map.values()); // ["some_key", "some_key", "some_other_key"]
    /// ```
    pub fn keys(&self) -> Vec<&String> {
        self.item_by_idx(0)
    }

    /// Return Vec of all values form MultiDict.
    ///
    /// # Examples
    ///
    /// ```
    /// use multidict::MultiDict;
    ///
    /// let mut map = MultiDict::new();
    /// map.add(["some_key".to_string(), "some_value_1".to_string()]);
    /// map.add(["some_key".to_string(), "some_value_2".to_string()]);
    /// println!("{:?}", map.values()); // ["some_value_1", "some_value_2"]
    /// ```
    pub fn values(&self) -> Vec<&String> {
        self.item_by_idx(1)
    }

    /// Update the MultiDict with the key/value pairs,
    /// overwriting existing keys/values
    ///
    /// # Examples
    ///
    /// This function update values *only* for already exists keys
    /// ```
    /// use multidict::MultiDict;
    ///
    /// let mut map = MultiDict::new();
    /// map.add(["some_key".to_string(), "some_value".to_string()]);
    /// map.update(["some_other_key".to_string(), "some_other_value".to_string()]);
    /// println!("{map:?}"); // MultiDict { elements: [["some_key", "some_value"]] }
    /// map.update(["some_key".to_string(), "some_other_value".to_string()]);
    /// println!("{map:?}"); // MultiDict { elements: [["some_key", "some_other_value"]] }
    /// ```
    ///
    /// And it's update all equal keys values
    /// ```
    /// use multidict::MultiDict;
    ///
    /// let mut map = MultiDict::new();
    /// map.add(["some_key".to_string(), "some_value_1".to_string()]);
    /// map.add(["some_key".to_string(), "some_value_2".to_string()]);
    /// println!("{map:?}"); // MultiDict { elements: [["some_key", "some_value_1"], ["some_key", "some_value_2"]] }
    /// map.update(["some_key".to_string(), "some_other_value".to_string()]);
    /// println!("{map:?}"); // MultiDict { elements: [["some_key", "some_other_value"], ["some_key", "some_other_value"]] }
    /// ```
    pub fn update(&mut self, new_item: [String; 2]) {
        let new_item_key = new_item[0].clone();
        let mut ids_for_replace = Vec::new();

        for (idx, item) in self.elements.iter().enumerate() {
            if item.get(0).unwrap().eq(&new_item_key) {
                ids_for_replace.push(idx);
            }
        }
        for idx in ids_for_replace {
            self.elements.remove(idx);
            self.elements.insert(idx, new_item.clone());
        }
    }
    fn item_by_idx(&self, idx: usize) -> Vec<&String> {
        let mut results = Vec::with_capacity(self.elements.len());
        for item in &self.elements {
            results.push(item.get(idx).unwrap());
        }
        results
    }
}
