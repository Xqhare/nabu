use super::XffValue;
use std::{collections::{BTreeMap, HashMap}, ops::Index};

#[derive(Debug, Clone, PartialEq)]
/// An object made up of key-value pairs of XFF values with string key with string keys.
///
/// Can be crated with `Object::from()` or `Object::new()`.
///
/// Most functionality needed for interacting with the underlying `BTreeMap` is provided on the
/// struct itself.
/// Access to the underlying map is provided through the `into_btree_map()` method.
///
/// `Object` implements `From<BTreeMap<String, XffValue>>`, `From<HashMap<String, XffValue>>`, and `From<Vec<(String, XffValue)>>`.
///
/// # Examples
/// ```rust
/// use std::collections::BTreeMap;
///
/// use nabu::{XffValue, Object};
///
/// let key_values = vec![
///     ("key0".to_string(), XffValue::from("hi mom!")),
///     ("key1".to_string(), XffValue::from(42.69)),
///     ("key2".to_string(), XffValue::from(42.69)),
/// ];
///
/// let mut object1 = Object::from(key_values);
///
/// assert_eq!(object1.len(), 3);
/// object1.clear();
/// assert!(object1.is_empty());
///
/// let mut object2 = Object::new();
/// object2.insert("keyA".to_string(), XffValue::from("hello mom!"));
/// object2.insert("keyB".to_string(), XffValue::from(420.69));
/// object2.insert("keyC".to_string(), XffValue::from(420.69));
///
/// let key_a = object2.remove("keyA");
/// assert_eq!(key_a, Some(XffValue::from("hello mom!")));
///
/// let key_b = object2.get("keyB");
/// assert_eq!(key_b, Some(&XffValue::from(420.69)));
///
///
/// assert!(object2.contains_key("keyC"));
///
/// for (key, value) in object2.iter() {
///     println!("{}: {:?}", key, value);
/// }
/// ```
///
/// ```rust
/// use std::collections::BTreeMap;
///
/// use nabu::{XffValue, Object};
///
/// let key_values = vec![
///     ("keyA".to_string(), XffValue::from("hi mom!")),
///     ("keyB".to_string(), XffValue::from(42.69)),
///     ("keyC".to_string(), XffValue::from(42.69)),
/// ];
///
/// let xff_obj_value = XffValue::from(key_values);
/// assert!(xff_obj_value.is_object());
///
/// let mut object = xff_obj_value.into_object().unwrap();
///
/// assert!(object.len() == 3);
///
/// let key_a = object.remove("keyA");
/// assert_eq!(key_a, Some(XffValue::from("hi mom!")));
///
/// assert!(object.len() == 2);
///
/// object.insert("keyA".to_string(), XffValue::from("hi mom!"));
///
/// assert!(object.len() == 3);
///
/// let key_b = object.get("keyB");
/// assert_eq!(key_b, Some(&XffValue::from(42.69)));
///
/// assert!(object.contains_key("keyB"));
///
/// for (key, value) in object.iter() {
///     println!("{}: {:?}", key, value);
/// }
/// ```
pub struct Object {
    /// An object of XFF values and string keys
    pub map: BTreeMap<String, XffValue>,
}

// -----------------------------------------------------------
//                     Convenience functions
// -----------------------------------------------------------

impl Object {
    /// Creates an empty object
    pub fn new() -> Self {
        Object {
            map: BTreeMap::new(),
        }
    }

    /// Convert the object into a BTreeMap.\
    /// Alternatively, use `Object::into_hash_map()`.
    ///
    /// # Example
    /// ```rust
    /// use std::collections::BTreeMap;
    ///
    /// use nabu::{XffValue, Object};
    ///
    /// let xff_obj_value = Object::from(vec![
    ///     ("keyA".to_string(), XffValue::from("hi mom!")),
    ///     ("keyB".to_string(), XffValue::from(42.69)),
    /// ]);
    ///
    /// let map: BTreeMap<String, XffValue> = xff_obj_value.into_btree_map();
    /// ```
    pub fn into_btree_map(self) -> BTreeMap<String, XffValue> {
        self.map
    }

    /// Convert the object into a HashMap.\
    /// Alternatively, use `Object::into_btree_map()`.
    ///
    /// # Example
    /// ```rust
    /// use std::collections::HashMap;
    ///
    /// use nabu::{XffValue, Object};
    ///
    /// let xff_obj_value = Object::from(vec![
    ///     ("keyA".to_string(), XffValue::from("hi mom!")),
    ///     ("keyB".to_string(), XffValue::from(42.69)),
    /// ]);
    ///
    /// let map: HashMap<String, XffValue> = xff_obj_value.into_hash_map();
    /// assert_eq!(map.len(), 2);
    /// ```
    pub fn into_hash_map(self) -> HashMap<String, XffValue> {
        self.map
            .into_iter()
            .map(|(k, v)| (k, v))
            .collect::<HashMap<String, XffValue>>()
    }

    /// Returns `true` if the object is empty
    ///
    /// # Example
    /// ```rust
    /// use nabu::{XffValue, Object};
    ///
    /// let mut xff_obj_value = Object::from(vec![
    ///     ("keyA".to_string(), XffValue::from("hi mom!")),
    ///     ("keyB".to_string(), XffValue::from(42.69)),
    /// ]);
    ///
    /// assert!(!xff_obj_value.is_empty());
    ///
    /// xff_obj_value.clear();
    ///
    /// assert!(xff_obj_value.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Clears the object
    ///
    /// # Example
    /// ```rust
    /// use nabu::{XffValue, Object};
    ///
    /// let mut xff_obj_value = Object::from(vec![
    ///     ("keyA".to_string(), XffValue::from("hi mom!")),
    ///     ("keyB".to_string(), XffValue::from(42.69)),
    /// ]);
    ///
    /// assert!(!xff_obj_value.is_empty());
    ///
    /// xff_obj_value.clear();
    ///
    /// assert!(xff_obj_value.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// Inserts a key-value pair into the object
    ///
    /// # Example
    /// ```rust
    /// use nabu::{XffValue, Object};
    ///
    /// let mut xff_obj_value = Object::new();
    ///
    /// xff_obj_value.insert("keyA".to_string(), XffValue::from("hi mom!"));
    /// xff_obj_value.insert("keyB".to_string(), XffValue::from(42.69));
    ///
    /// assert_eq!(xff_obj_value.len(), 2);
    /// ```
    pub fn insert(&mut self, key: String, value: XffValue) {
        self.map.insert(key, value);
    }

    /// Removes a key-value pair from the object
    ///
    /// # Example
    /// ```rust
    /// use nabu::{XffValue, Object};
    ///
    /// let mut xff_obj_value = Object::new();
    ///
    /// xff_obj_value.insert("keyA".to_string(), XffValue::from("hi mom!"));
    /// xff_obj_value.insert("keyB".to_string(), XffValue::from(42.69));
    ///
    /// assert_eq!(xff_obj_value.len(), 2);
    ///
    /// let key_a = xff_obj_value.remove("keyA");
    /// assert_eq!(key_a, Some(XffValue::from("hi mom!")));
    ///
    /// assert_eq!(xff_obj_value.len(), 1);
    /// ```
    pub fn remove(&mut self, key: &str) -> Option<XffValue> {
        self.map.remove(key)
    }

    /// Returns a reference to the value associated with the key
    ///
    /// # Example
    /// ```rust
    /// use nabu::{XffValue, Object};
    ///
    /// let xff_obj_value = Object::from(vec![
    ///     ("keyA".to_string(), XffValue::from("hi mom!")),
    ///     ("keyB".to_string(), XffValue::from(42.69)),
    /// ]);
    ///
    /// assert_eq!(xff_obj_value.get("keyA"), Some(&XffValue::from("hi mom!")));
    /// assert_eq!(xff_obj_value.get("keyB"), Some(&XffValue::from(42.69)));
    /// assert_eq!(xff_obj_value.get("keyC"), None);
    /// ```
    pub fn get(&self, key: &str) -> Option<&XffValue> {
        self.map.get(key)
    }

    /// Returns the number of key-value pairs in the object, also known as its length.
    ///
    /// # Example
    /// ```rust
    /// use nabu::{XffValue, Object};
    ///
    /// let xff_obj_value = Object::from(vec![
    ///     ("keyA".to_string(), XffValue::from("hi mom!")),
    ///     ("keyB".to_string(), XffValue::from(42.69)),
    /// ]);
    ///
    /// assert_eq!(xff_obj_value.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns `true` if the object contains the supplied key
    ///
    /// # Example
    /// ```rust
    /// use nabu::{XffValue, Object};
    ///
    /// let xff_obj_value = Object::from(vec![
    ///     ("keyA".to_string(), XffValue::from("hi mom!")),
    ///     ("keyB".to_string(), XffValue::from(42.69)),
    /// ]);
    ///
    /// assert!(xff_obj_value.contains_key("keyA"));
    /// assert!(!xff_obj_value.contains_key("keyC"));
    /// ```
    pub fn contains_key(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    /// Returns a key-value pair iterator of the object
    ///
    /// # Example
    /// ```rust
    /// use nabu::{XffValue, Object};
    ///
    /// let xff_obj_value = Object::from(vec![
    ///     ("keyA".to_string(), XffValue::from("hi mom!")),
    ///     ("keyB".to_string(), XffValue::from(42.69)),
    /// ]);
    ///
    /// for (key, value) in xff_obj_value.iter() {
    ///     println!("{}: {}", key, value);
    /// }
    /// ```
    pub fn iter(&self) -> std::collections::btree_map::Iter<'_, String, XffValue> {
        self.map.iter()
    }
}

// -----------------------------------------------------------
//                     From implementations
// -----------------------------------------------------------

impl<S, V> From<Vec<(S, V)>> for Object where S: Into<String>, V: Into<XffValue> {
    fn from(vec: Vec<(S, V)>) -> Self {
        Object {
            map: vec.into_iter().map(|(k, v)| (k.into(), v.into())).collect(),
        }
    }
}

impl<S, V> From<HashMap<S, V>> for Object where S: Into<String>, V: Into<XffValue> {
    fn from(map: HashMap<S, V>) -> Self {
        let mut out: BTreeMap<String, XffValue> = BTreeMap::new();
        for (k, v) in map {
            out.insert(k.into(), v.into());
        }
        Object { map: out }
    }
}

impl<S, V> From<BTreeMap<S, V>> for Object where S: Into<String>, V: Into<XffValue> {
    fn from(map: BTreeMap<S, V>) -> Self {
        let mut out: BTreeMap<String, XffValue> = BTreeMap::new();
        for (k, v) in map {
            out.insert(k.into(), v.into());
        }
        Object { map: out }
    }
}

// -----------------------------------------------------------
//                     Index implementations
// -----------------------------------------------------------

impl<S> Index<S> for Object where S: AsRef<str> {
    type Output = XffValue;

    fn index(&self, index: S) -> &Self::Output {
        self.get(index.as_ref()).unwrap()
    }
}

// -----------------------------------------------------------
//                     Display implementation
// -----------------------------------------------------------

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{")?;
        for (i, (key, value)) in self.map.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", key, value)?;
        }
        write!(f, "}}")
    }
}
