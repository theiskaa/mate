//
// Copyright 2022-present theiskaa. All rights reserved.
// Use of this source code is governed by MIT license
// that can be found in the LICENSE file.
//

use std::collections::HashMap;

/// Environment holds variable bindings for the calculator.
/// Variables are stored as name -> value mappings.
#[derive(Clone, Debug, Default)]
pub struct Environment {
    variables: HashMap<String, f64>,
}

impl Environment {
    /// Creates a new empty environment.
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    /// Sets a variable to a value.
    pub fn set(&mut self, name: &str, value: f64) {
        self.variables.insert(name.to_string(), value);
    }

    /// Gets a variable's value, if it exists.
    pub fn get(&self, name: &str) -> Option<f64> {
        self.variables.get(name).copied()
    }

    /// Checks if a variable exists.
    pub fn exists(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }

    /// Returns all variable names.
    pub fn names(&self) -> Vec<&String> {
        self.variables.keys().collect()
    }

    /// Clears all variables.
    pub fn clear(&mut self) {
        self.variables.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_environment() {
        let env = Environment::new();
        assert!(env.variables.is_empty());
    }

    #[test]
    fn set_and_get() {
        let mut env = Environment::new();
        env.set("x", 5.0);
        assert_eq!(env.get("x"), Some(5.0));
        assert_eq!(env.get("y"), None);
    }

    #[test]
    fn overwrite_variable() {
        let mut env = Environment::new();
        env.set("x", 5.0);
        env.set("x", 10.0);
        assert_eq!(env.get("x"), Some(10.0));
    }

    #[test]
    fn exists() {
        let mut env = Environment::new();
        env.set("x", 5.0);
        assert!(env.exists("x"));
        assert!(!env.exists("y"));
    }

    #[test]
    fn clear() {
        let mut env = Environment::new();
        env.set("x", 5.0);
        env.set("y", 10.0);
        env.clear();
        assert!(!env.exists("x"));
        assert!(!env.exists("y"));
    }
}
