//! Trie

use std::collections::HashMap;
use std::hash::Hash;

/// Trie datastructure with algorithms that can be performed on a trie
/// Trie should also implement IntoIterator and remove
pub trait Trie<K, V> {
    fn get(&self, key: K) -> Option<&V>;
    fn insert(&mut self, key: K, value: V) -> Option<V>;
}

struct ElementaryElement<KE, V>
    where KE: Eq + Hash {
    value: Option<V>,
    children: HashMap<KE, ElementaryElement<KE, V>>
}

pub struct ElementaryTrie<KE, V>
    where KE: Eq + Hash {
    root: ElementaryElement<KE, V>
}

impl<KE, V> ElementaryTrie<KE, V>
    where KE: Eq + Hash {

    pub fn new() -> ElementaryTrie<KE, V> {
        ElementaryTrie {
            root: ElementaryTrie::new_elementary_element(),
        }
    }

    fn new_elementary_element() -> ElementaryElement<KE, V> {
        ElementaryElement {
            value: None,
            children: HashMap::new(),
        }
    }

    /// Here the recursion keeps track of the lifetimes of the elements as the tree is descended -
    /// it seems a waste and probably can be elimitnated with proper lifetime management
    fn get_or_create_element<'a, K>(element: &'a mut ElementaryElement<KE, V>, mut key: K) -> &'a mut ElementaryElement<KE, V> 
        where K: Iterator<Item=KE> {
        if let Some(k) = key.next() {
            let child = element.children.entry(k).or_insert(ElementaryTrie::new_elementary_element());
            ElementaryTrie::get_or_create_element(child, key)
        } else {
            element
        }
    }
}

impl<K, KE, V> Trie<K, V> for ElementaryTrie<KE, V> 
    where K: std::iter::IntoIterator<Item=KE>,
          KE: Eq + Hash {

    fn get(&self, key: K) -> Option<&V> {
        let mut element = &self.root;
        for e in key.into_iter() {
            if let Some(e) = element.children.get(&e) {
                element = e;
            } else {
                return None
            }
        }
        element.value.as_ref()
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let mut alt = Some(value);
        let element = ElementaryTrie::get_or_create_element(&mut self.root, key.into_iter());
        std::mem::swap(&mut element.value, &mut alt);
        alt
    }

}

#[cfg(test)]
mod vec_tests {
    use super::*;

    #[test]
    fn route_element_insertion_and_retrieval() {
        let mut trie: ElementaryTrie<char, u8> = ElementaryTrie::new();
        assert_eq!(None, trie.insert(vec![], 100));
        assert_eq!(Some(&100), trie.get(vec![]));
    }

    #[test]
    fn insertion_replaces_previous_value() {
        let mut trie: ElementaryTrie<char, u8> = ElementaryTrie::new();
        assert_eq!(None, trie.insert(vec![], 100));
        assert_eq!(Some(100), trie.insert(vec![], 101));
        assert_eq!(Some(&101), trie.get(vec![]));
    }

    #[test]
    fn multiple_route_element_insertion_and_retrieval() {
        let mut trie: ElementaryTrie<char, u8> = ElementaryTrie::new();
        assert_eq!(None, trie.insert(vec![], 1));
        assert_eq!(None, trie.insert(vec!['a'], 2));
        assert_eq!(None, trie.insert(vec!['b'], 3));
        assert_eq!(Some(&1), trie.get(vec![]));
        assert_eq!(Some(&2), trie.get(vec!['a']));
        assert_eq!(Some(&3), trie.get(vec!['b']));
    }

    #[test]
    fn child_route_element_insertion_and_retrieval() {
        let mut trie: ElementaryTrie<char, u8> = ElementaryTrie::new();
        assert_eq!(None, trie.insert(vec![], 1));
        assert_eq!(None, trie.insert(vec!['a'], 2));
        assert_eq!(None, trie.insert(vec!['a', 'b'], 3));
        assert_eq!(Some(&1), trie.get(vec![]));
        assert_eq!(Some(&2), trie.get(vec!['a']));
        assert_eq!(Some(&3), trie.get(vec!['a', 'b']));
    }

    #[test]
    fn orphan_element_insertion_and_retrieval() {
        let mut trie: ElementaryTrie<char, u8> = ElementaryTrie::new();
        assert_eq!(None, trie.insert(vec![], 1));
        assert_eq!(None, trie.insert(vec!['a', 'b'], 3));
        assert_eq!(Some(&1), trie.get(vec![]));
        assert_eq!(None, trie.get(vec!['a']));
        assert_eq!(Some(&3), trie.get(vec!['a', 'b']));
    }
}

