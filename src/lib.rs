//! # multidict
//!
//! The `multidict` crate was inspired by Python `MultiDict` library
//! `multidict` is useful for working with HTTP headers, URL query, form-data args etc
//! HTTP Headers and URL query string require specific data structure: `multidict`.
//! It behaves mostly like a regular map but it may have several values for the same key
//! and preserves insertion ordering
//!
//! ## Examples
//!
//! Basic `MultiDict` creation and filling with data
//! ```
//! use multidict::{MultiDict, MultiElement};
//!
//! let mut map = MultiDict::new();
//! map.add(MultiElement {
//!             key: "some_key".to_string(),
//!             value: "some_value_1".to_string(),
//!         });
//! println!("{map}"); // MultiDict < "some_key":"some_value_1" >
//! map.add(MultiElement {
//!             key: "some_key".to_string(),
//!             value: "some_value_2".to_string(),
//!         });
//! println!("{map}");
//! // MultiDict < "some_key":"some_value_1", "some_key":"some_value_2" >
//! ```
//!
//! Get **all** key-values pairs for key
//! ```
//! use multidict::{MultiDict, MultiElement};
//!
//! let mut map = MultiDict::new();
//! map.add(MultiElement {
//!             key: "some_key".to_string(),
//!             value: "some_value_1".to_string(),
//!         });
//! map.add(MultiElement {
//!             key: "some_key".to_string(),
//!             value: "some_value_2".to_string(),
//!         });
//! map.add(MultiElement {
//!             key: "some_other_key".to_string(),
//!             value: "some_value_3".to_string(),
//!         });
//! println!("{}", map.getall("some_key").unwrap());
//! // MultiDict < "some_key":"some_value_1", "some_key":"some_value_2" >
//! ```
//!
//! `MultiDict` debug output
//! ```
//! use multidict::{MultiDict, MultiElement};
//!
//! let mut map = MultiDict::new();
//! map.add(MultiElement {
//!             key: "some_key".to_string(),
//!             value: "some_value_1".to_string(),
//!         });
//! map.add(MultiElement {
//!             key: "some_key".to_string(),
//!             value: "some_value_2".to_string(),
//!         });
//! map.add(MultiElement {
//!             key: "some_other_key".to_string(),
//!             value: "some_value_3".to_string(),
//!         });
//! println!("{map:?}");
//! // MultiDict { elements: [
//! //  MultiElement { key: "some_key", value: "some_value_1" },
//! //  MultiElement { key: "some_key", value: "some_value_2" },
//! //  MultiElement { key: "some_other_key", value: "some_value_3" }
//! // ] }
//! ```

use std::fmt;

/// `MultiElement` - element of `MultiDict` structure Vec.
#[derive(Debug, Clone)]
pub struct MultiElement {
    pub key: String,
    pub value: String,
}
impl fmt::Display for MultiElement {
    /// `MultiElement` instance formatter
    ///
    /// # Examples
    /// ```
    /// use multidict::MultiElement;
    ///
    /// let element: MultiElement = MultiElement::new(["some_key".to_string(),
    ///                                                 "some_value".to_string()
    ///                                             ]);
    /// println!("{element}")
    /// // MultiElement < "some_key":"some_value_2" >
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"MultiElement < "{}":"{}" >"#, self.key, self.value)
    }
}
impl MultiElement {
    /// Return new MultiElement instance
    ///
    /// # Examples
    /// ```
    /// use multidict::MultiElement;
    ///
    /// let element: MultiElement = MultiElement::new(["some_key".to_string(),
    ///                                                 "some_value".to_string()
    ///                                             ]);
    /// ```
    pub fn new(new_element: [String; 2]) -> Self {
        MultiElement {
            key: new_element[0].clone(),
            value: new_element[1].clone(),
        }
    }
}

/// `MultiDict` - structure which enable to user store multiple
/// similar keys with different values in map-like structure.
///
/// Was inspired by Python `MultiDict` library
#[derive(Default, Debug, Clone)]
pub struct MultiDict {
    pub elements: Vec<MultiElement>,
}
impl fmt::Display for MultiDict {
    /// `MultiDict` instance formatter
    ///
    /// # Examples
    /// ```
    /// use multidict::{MultiDict, MultiElement};
    ///
    /// let mut map: MultiDict = MultiDict::new();
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_1".to_string(),
    ///         });
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_2".to_string(),
    ///         });
    /// println!("{}", map);
    /// // MultiDict < "some_key":"some_value_1", "some_key":"some_value_2" >
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MultiDict < {} >",
            self.elements
                .iter()
                .map(|item| format!(r#""{}":"{}""#, item.key, item.value))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
impl MultiDict {
    /// Return new MultiDict instance
    ///
    /// # Examples
    /// ```
    /// use multidict::MultiDict;
    ///
    /// let mut map: MultiDict = MultiDict::new();
    /// ```
    pub fn new() -> Self {
        MultiDict {
            elements: Vec::new(),
        }
    }

    /// Return new MultiDict instance with preset capacity
    ///
    /// # Examples
    ///
    /// Capacity will only prepare Vector with element for future elements,
    /// but it can expand
    /// ```
    /// use multidict::{MultiDict, MultiElement};
    ///
    /// let mut map: MultiDict = MultiDict::new_capacity(&2);
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_1".to_string(),
    ///         });
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_2".to_string(),
    ///         });
    /// println!("{}", map.len());
    /// // 2
    /// ```
    /// ```
    /// use multidict::{MultiDict, MultiElement};
    ///
    /// let mut map: MultiDict = MultiDict::new_capacity(&2);
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_1".to_string(),
    ///         });
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_2".to_string(),
    ///         });
    /// map.add(MultiElement {
    ///             key: "some_other_key".to_string(),
    ///             value: "some_value_3".to_string(),
    ///         });
    /// println!("{}", map.len());
    /// // 3
    /// ```
    pub fn new_capacity(capacity: &usize) -> Self {
        MultiDict {
            elements: Vec::with_capacity(*capacity),
        }
    }

    /// Return the number of items in MultiDict
    ///
    /// # Examples
    /// ```
    /// use multidict::{MultiDict, MultiElement};
    ///
    /// let mut map = MultiDict::new();
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_1".to_string(),
    ///         });
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_2".to_string(),
    ///         });
    /// println!("{}", map.len());
    /// // 2
    /// ```
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Return true if `MultiDict` has **no** elements,
    /// else false
    ///
    /// # Examples
    ///
    /// When some elements is added
    /// ```
    /// use multidict::{MultiDict, MultiElement};
    ///
    /// let mut map = MultiDict::new();
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_1".to_string(),
    ///         });
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_2".to_string(),
    ///         });
    /// println!("{}", map.is_empty());
    /// // false
    /// ```
    /// ```
    /// use multidict::{MultiDict, MultiElement};
    ///
    /// let mut map = MultiDict::new();
    /// println!("{}", map.is_empty());
    /// // true
    /// ```
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
    /// Append (key, value) pair to the MultiDict.
    ///
    /// # Examples
    /// ```
    /// use multidict::{MultiDict, MultiElement};
    ///
    /// let mut map = MultiDict::new();
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_1".to_string(),
    ///         });
    /// println!("{map}");
    /// // MultiDict < "some_key":"some_value_1" >
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_2".to_string(),
    ///         });
    /// println!("{map}");
    /// // MultiDict < "some_key":"some_value_1", "some_key":"some_value_2" >
    /// ```
    pub fn add(&mut self, new_item: MultiElement) {
        self.elements.push(new_item);
    }

    /// Return the **first** key-value pair for key if key is in the MultiDict
    ///
    /// # Examples
    ///
    /// If key exists
    /// ```
    /// use multidict::{MultiDict, MultiElement};
    ///
    /// let mut map = MultiDict::new();
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_1".to_string(),
    ///         });
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_2".to_string(),
    ///         });
    /// println!("{}", map.get("some_key").unwrap());
    /// // MultiElement < "some_key":"some_value_1" >
    /// ```
    ///
    /// If key not exists
    /// ```
    /// use multidict::{MultiDict, MultiElement};
    ///
    /// let mut map = MultiDict::new();
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_1".to_string(),
    ///         });
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_2".to_string(),
    ///         });
    /// println!("{:?}", map.get("some_other_key"));
    /// // Err("No matching key found")
    /// ```
    pub fn get(&self, key: &str) -> Result<&MultiElement, &str> {
        for item in &self.elements {
            if item.key.eq(key) {
                return Ok(item);
            }
        }
        Err("No matching key found")
    }

    /// If key is in the MultiDict, remove it and return its the **first** value,
    /// else return error text
    ///
    /// # Examples
    ///
    /// If key exists
    /// ```
    /// use multidict::{MultiDict, MultiElement};
    ///
    /// let mut map = MultiDict::new();
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_1".to_string(),
    ///         });
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_2".to_string(),
    ///         });
    /// println!("{}", map);
    /// // MultiDict < "some_key":"some_value_1", "some_key":"some_value_2" >
    /// println!("{}", map.popone("some_key").unwrap());
    /// // MultiElement < "some_key":"some_value_1" >
    /// println!("{}", map);
    /// // MultiDict < "some_key":"some_value_2" >
    /// ```
    ///
    /// If key not exists
    /// ```
    /// use multidict::{MultiDict, MultiElement};
    ///
    /// let mut map = MultiDict::new();
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_1".to_string(),
    ///         });
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_2".to_string(),
    ///         });
    /// println!("{:?}", map.popone("some_other_key"));
    /// // Err("No matching key found")
    /// println!("{}", map);
    /// // MultiDict < "some_key":"some_value_1", "some_key":"some_value_2" >
    /// ```
    ///
    pub fn popone(&mut self, key: &str) -> Result<MultiElement, &str> {
        for (idx, item) in self.elements.iter().enumerate() {
            if item.key.eq(key) {
                return Ok(self.elements.remove(idx));
            }
        }
        Err("No matching key found")
    }

    /// Return a list of all key-values for key if key is in the MultiDict
    /// else - return error
    ///
    /// # Examples
    ///
    /// If key exists
    /// ```
    /// use multidict::{MultiDict, MultiElement};
    ///
    /// let mut map = MultiDict::new();
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_1".to_string(),
    ///         });
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_2".to_string(),
    ///         });
    /// println!("{}", map.getall("some_key").unwrap());
    /// // MultiDict < "some_key":"some_value_1", "some_key":"some_value_2" >
    /// ```
    ///
    /// If key not exists
    /// ```
    /// use multidict::{MultiDict, MultiElement};
    ///
    /// let mut map = MultiDict::new();
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_1".to_string(),
    ///         });
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_2".to_string(),
    ///         });
    /// println!("{:?}", map.getall("some_other_key")); // Err("No matching key found")
    /// ```
    pub fn getall(&self, key: &str) -> Result<MultiDict, &str> {
        let mut results = MultiDict::new();
        for item in &self.elements {
            if item.key.eq(key) {
                results.add(item.clone());
            }
        }
        if !results.is_empty() {
            Ok(results)
        } else {
            Err("No matching key found")
        }
    }

    /// Return True if MultiDict has a key, else False.
    ///
    /// # Examples
    ///
    /// If key exists
    /// ```
    /// use multidict::{MultiDict, MultiElement};
    ///
    /// let mut map = MultiDict::new();
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_1".to_string(),
    ///         });
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_2".to_string(),
    ///         });
    /// println!("{}", map.contains("some_key"));
    /// // true
    /// ```
    ///
    /// If key not exists
    /// ```
    /// use multidict::{MultiDict, MultiElement};
    ///
    /// let mut map = MultiDict::new();
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_1".to_string(),
    ///         });
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_2".to_string(),
    ///         });
    /// println!("{}", map.contains("some_other_key"));
    /// // false
    /// ```
    pub fn contains(&self, key: &str) -> bool {
        for item in &self.elements {
            if item.key.eq(key) {
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
    /// use multidict::{MultiDict, MultiElement};
    ///
    /// let mut map = MultiDict::new();
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_1".to_string(),
    ///         });
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_2".to_string(),
    ///         });
    /// map.add(MultiElement {
    ///             key: "some_other_key".to_string(),
    ///             value: "some_value_3".to_string(),
    ///         });
    /// println!("{:?}", map.keys());
    /// // ["some_key", "some_key", "some_other_key"]
    /// ```
    pub fn keys(&self) -> Vec<&String> {
        let mut results: Vec<&String> = Vec::with_capacity(self.elements.len());
        for item in &self.elements {
            results.push(&item.key);
        }
        results
    }

    /// Return Vec of all values form MultiDict.
    ///
    /// # Examples
    ///
    /// ```
    /// use multidict::{MultiDict, MultiElement};
    ///
    /// let mut map = MultiDict::new();
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_1".to_string(),
    ///         });
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_2".to_string(),
    ///         });
    /// map.add(MultiElement {
    ///             key: "some_other_key".to_string(),
    ///             value: "some_value_3".to_string(),
    ///         });
    /// println!("{:?}", map.values());
    /// // ["some_value_1", "some_value_2", "some_value_3"]
    /// ```
    pub fn values(&self) -> Vec<&String> {
        let mut results: Vec<&String> = Vec::with_capacity(self.elements.len());
        for item in &self.elements {
            results.push(&item.value);
        }
        results
    }

    /// Update the MultiDict with the key/value pairs,
    /// overwriting existing keys/values
    ///
    /// # Examples
    ///
    /// This function update values **only** for already exists keys
    /// ```
    /// use multidict::{MultiDict, MultiElement};
    ///
    /// let mut map = MultiDict::new();
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_1".to_string(),
    ///         });
    /// map.add(MultiElement {
    ///             key: "some_other_key".to_string(),
    ///             value: "some_value_2".to_string(),
    ///         });
    /// println!("{map}");
    /// // MultiDict < "some_key":"some_value_1", "some_other_key":"some_value_2" >
    /// map.update(MultiElement {
    ///             key: "some_other_key".to_string(),
    ///             value: "some_value_3".to_string(),
    ///         });
    /// println!("{map}");
    /// // MultiDict < "some_key":"some_value_1", "some_other_key":"some_value_3" >
    /// ```
    ///
    /// And it's update all equal keys values
    /// ```
    /// use multidict::{MultiDict, MultiElement};
    ///
    /// let mut map = MultiDict::new();
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_1".to_string(),
    ///         });
    /// map.add(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_2".to_string(),
    ///         });
    /// println!("{map}");
    /// // MultiDict < "some_key":"some_value_1", "some_key":"some_value_2" >
    /// map.update(MultiElement {
    ///             key: "some_key".to_string(),
    ///             value: "some_value_3".to_string(),
    ///         });
    /// println!("{map}");
    /// // MultiDict < "some_key":"some_value_3", "some_key":"some_value_3" >
    /// ```
    pub fn update(&mut self, new_item: MultiElement) {
        let new_item_key = &new_item.key;
        let mut ids_for_replace = Vec::new();

        for (idx, item) in self.elements.iter().enumerate() {
            if item.key.eq(new_item_key) {
                ids_for_replace.push(idx);
            }
        }
        for idx in ids_for_replace {
            self.elements.remove(idx);
            self.elements.insert(idx, new_item.clone());
        }
    }
}
