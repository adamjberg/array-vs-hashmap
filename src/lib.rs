#![feature(test)]

use std::collections::HashMap;

extern crate test;

struct KeyValueStore {
  keys: Vec<String>,
  vals: Vec<String>,
  len: usize
}

impl KeyValueStore {
  fn insert(& mut self, key: String, val: String) {
      self.keys.push(key);
      self.vals.push(val);
      self.len += 1;
  }

  fn get(&self, key: &str) -> Option<&String> {
      for i in 0..self.len {
          if self.keys[i] == key {
              return Some(&self.vals[i]);
          }
      }
      return None;
  }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_kv_store(b: &mut Bencher) {
      let mut kv_store = KeyValueStore {
          keys: vec![],
          vals: vec![],
          len: 0
      };

      for i in 1..22 {
        let k = i.to_string();
        let v =  i.to_string();

        kv_store.insert(k.clone(), v.clone());
      }

      b.iter(|| kv_store.get("22"));
    }

    #[bench]
    fn bench_hashmap(b: &mut Bencher) {
      let mut hashmap = HashMap::new();

      for i in 1..22 {
          let k = i.to_string();
          let v =  i.to_string();

          hashmap.insert(k.clone(), v.clone());
      }

      b.iter(|| hashmap.get("22"));
    }
}