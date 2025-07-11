#![feature(specialization)]

use std::sync::Arc;

use pi_ordmap::{ordmap::{ImOrdMap, OrdMap},
                asbtree::{TreeByteSize, Tree}};

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Binary(Arc<Vec<u8>>);

impl Clone for Binary {
    fn clone(&self) -> Self {
        Binary(self.0.clone())
    }
}

impl From<&str> for Binary {
    fn from(value: &str) -> Self {
        Binary(Arc::new(value.as_bytes().to_vec()))
    }
}

impl TreeByteSize for Binary {
    fn tree_bytes_size(&self) -> u64 {
        self.0.len() as u64
    }
}

#[test]
fn test_bytes_size_with_sbtree() {
    let mut tree: OrdMap<Tree<Binary, Binary>> = OrdMap::new(Tree::new());
    assert_eq!(tree.bytes_size(), 0);
    tree.insert("abc".into(), "abc".into());
    assert_eq!(tree.bytes_size(), 6);
    let r = tree.upsert("abc".into(), "abcabc".into(), true);
    assert!(r.is_some());
    assert_eq!(tree.bytes_size(), 9);
    let r = tree.upsert("abc".into(), "abc".into(), false);
    assert!(r.is_some());
    assert_eq!(tree.bytes_size(), 6);
    let r = tree.delete(&"abc".into(), false);
    assert!(r.is_some());
    assert_eq!(tree.bytes_size(), 0);

    //插入
    let mut bytes_size = 0;
    for index in 0..8192 {
        let key = "abc".repeat(index + 1);
        let value = "abc".repeat(index + 1);
        let _ = tree.upsert(key.as_str().into(), value.as_str().into(), false);
        bytes_size += Binary::from(key.as_str()).tree_bytes_size() + Binary::from(value.as_str()).tree_bytes_size();
        assert_eq!(tree.bytes_size(), bytes_size);
    }

    //更新
    let mut tmp_bytes_size = bytes_size;
    for index in 0..8192 {
        let key = "abc".repeat(index + 1);
        let value = "abc".to_string();
        let _ = tree.upsert(key.as_str().into(), value.as_str().into(), false);
        tmp_bytes_size -= Binary::from(key.as_str()).tree_bytes_size() - 3;
        assert_eq!(tree.bytes_size(), tmp_bytes_size);
    }
    for index in (0..8192).rev() {
        let key = "abc".repeat(index + 1);
        let value = "abc".repeat(index + 1);
        let _ = tree.upsert(key.as_str().into(), value.as_str().into(), false);
        tmp_bytes_size += Binary::from(value.as_str()).tree_bytes_size() - 3;
        assert_eq!(tree.bytes_size(), tmp_bytes_size);
    }

    //删除
    for index in (0..8192).rev() {
        let key = "abc".repeat(index + 1);
        let value = "abc".repeat(index + 1);
        let r = tree.delete(&key.as_str().into(), false);
        assert!(r.is_some());
        bytes_size -= Binary::from(key.as_str()).tree_bytes_size() + Binary::from(value.as_str()).tree_bytes_size();
        assert_eq!(tree.bytes_size(), bytes_size);
    }
}