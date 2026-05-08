use std::{
  convert::Infallible,
  error::Error,
  fmt::{Debug, Display},
  ops::Deref,
};

pub struct UnionFindMergeError<T, E> {
  user_error: E,
  other: T,
}

impl<T, E> UnionFindMergeError<T, E> {
  pub fn new(user_error: E, other: T) -> Self {
    Self { user_error, other }
  }
}

impl<T, E: Debug> Debug for UnionFindMergeError<T, E> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self.user_error)
  }
}
impl<T, E: Display> Display for UnionFindMergeError<T, E> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.user_error)
  }
}
impl<T, E: Error> Error for UnionFindMergeError<T, E> {}

pub trait UnionFindData: Sized {
  type Error: Error;

  fn merge(&mut self, other: Self) -> Result<(), UnionFindMergeError<Self, Self::Error>>;
}

impl UnionFindData for () {
  type Error = Infallible;

  fn merge(&mut self, _other: ()) -> Result<(), UnionFindMergeError<(), Infallible>> {
    Ok(())
  }
}

#[derive(Clone, Copy)]
struct Node<T> {
  /// The index of parent of this node (self if root).
  parent: usize,
  data: Option<T>,
}

pub struct RootNodeProxy<'a, T> {
  union_find: &'a UnionFind<T>,
  id: usize,
}

impl<'a, T> RootNodeProxy<'a, T> {
  pub fn id(&self) -> usize {
    self.id
  }

  pub fn data(&self) -> &T {
    let data_option = self.union_find.get_node(self.id).data.as_ref();
    unsafe { data_option.unwrap_unchecked() }
  }
}

pub struct RootNodeMutProxy<'a, T> {
  union_find: &'a mut UnionFind<T>,
  id: usize,
}

impl<'a, T> RootNodeMutProxy<'a, T> {
  pub fn id(&self) -> usize {
    self.id
  }

  pub fn data(&mut self) -> &T {
    let data_option = self.union_find.get_node(self.id).data.as_ref();
    unsafe { data_option.unwrap_unchecked() }
  }

  pub fn data_mut(&mut self) -> &mut T {
    let data_option = self.union_find.get_node_mut(self.id).data.as_mut();
    unsafe { data_option.unwrap_unchecked() }
  }
}

impl<'a, T> From<RootNodeMutProxy<'a, T>> for RootNodeProxy<'a, T> {
  fn from(value: RootNodeMutProxy<'a, T>) -> Self {
    RootNodeProxy {
      union_find: value.union_find,
      id: value.id,
    }
  }
}

impl<'a, T> Deref for RootNodeProxy<'a, T> {
  type Target = T;

  fn deref(&self) -> &T {
    self.data()
  }
}

impl<'a, T: Debug> Debug for RootNodeProxy<'a, T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Node id {} (data = {:?})", self.id, self.data())
  }
}

pub struct UnionFind<T> {
  unique_sets: usize,
  elements: Vec<Node<T>>,
}

impl UnionFind<()> {
  pub fn new(capacity: usize) -> Self {
    let elements = (0..capacity)
      .enumerate()
      .map(|(idx, _)| Node {
        parent: idx,
        data: Some(()),
      })
      .collect();

    Self {
      unique_sets: capacity,
      elements,
    }
  }
}

impl<T> Default for UnionFind<T> {
  fn default() -> Self {
    Self {
      unique_sets: Default::default(),
      elements: Default::default(),
    }
  }
}

impl<T> UnionFind<T> {
  pub fn capacity(&self) -> usize {
    self.elements.len()
  }

  pub fn unique_sets(&self) -> usize {
    self.unique_sets
  }

  fn get_node(&self, node_id: usize) -> &Node<T> {
    debug_assert!(node_id < self.capacity());
    unsafe { self.elements.get_unchecked(node_id) }
  }

  fn get_node_mut(&mut self, node_id: usize) -> &mut Node<T> {
    debug_assert!(node_id < self.capacity());
    unsafe { self.elements.get_unchecked_mut(node_id) }
  }

  /// Gives a mutable proxy to the root of tree that node_id is in.
  pub fn find_mut(&mut self, mut node_id: usize) -> RootNodeMutProxy<'_, T> {
    debug_assert!(node_id < self.capacity());

    while self.get_node(node_id).parent != node_id {
      let node = self.get_node(node_id);
      let parent_id = node.parent;
      let grandparent_id = self.get_node(parent_id).parent;
      // Slowly compress tree by assigning node's parent to its grandparent.
      unsafe {
        self.elements.get_unchecked_mut(node_id).parent = grandparent_id;
      }

      // Next look at the former parent of node, rather than skipping to it's
      // new parent. This will cause a long chain of nodes to be compressed into
      // two equally-sized trees.
      node_id = parent_id;
    }

    RootNodeMutProxy {
      union_find: self,
      id: node_id,
    }
  }

  /// Gives a proxy to the root of tree that node_id is in.
  pub fn find(&mut self, node_id: usize) -> RootNodeProxy<'_, T> {
    self.find_mut(node_id).into()
  }

  /// Gives id of the root of tree that node is in. Does not do path compression.
  pub fn const_find(&self, mut node_id: usize) -> RootNodeProxy<'_, T> {
    debug_assert!(node_id < self.capacity());

    while self.get_node(node_id).parent != node_id {
      node_id = self.get_node(node_id).parent;
    }

    RootNodeProxy {
      union_find: self,
      id: node_id,
    }
  }

  /// Adds a new singleton set with `data`.
  pub fn add_set(&mut self, data: T) -> RootNodeProxy<'_, T> {
    let id = self.elements.len();
    self.elements.push(Node {
      parent: id,
      data: Some(data),
    });
    self.unique_sets += 1;

    RootNodeProxy {
      union_find: self,
      id,
    }
  }
}

impl<T: UnionFindData> UnionFind<T> {
  pub fn new_with_data(data: impl Into<Vec<T>>) -> Self {
    let data = data.into();
    let unique_sets = data.len();
    let elements = data
      .into_iter()
      .enumerate()
      .map(|(idx, data)| Node {
        parent: idx,
        data: Some(data),
      })
      .collect();

    Self {
      unique_sets,
      elements,
    }
  }

  /// Unions the two sets that a and b are in (noop if are already in the same
  /// set), returning the new set index of the two nodes.
  pub fn try_union(&mut self, a_id: usize, b_id: usize) -> Result<RootNodeProxy<'_, T>, T::Error> {
    let a_root_id = self.find(a_id).id;
    let b_root_id = self.find(b_id).id;

    if a_root_id != b_root_id {
      let b_node = self.get_node_mut(b_root_id);

      debug_assert!(b_node.data.is_some());
      let b_data = unsafe { b_node.data.take().unwrap_unchecked() };

      let a_node = self.get_node_mut(a_root_id);
      let a_data = unsafe { a_node.data.as_mut().unwrap_unchecked() };
      if let Err(UnionFindMergeError { user_error, other }) = a_data.merge(b_data) {
        let b_node = self.get_node_mut(b_root_id);
        b_node.data.replace(other);
        return Err(user_error);
      }

      let b_node = self.get_node_mut(b_root_id);
      b_node.parent = a_root_id;

      // Two sets have joined, reducing the number of unique sets by one.
      self.unique_sets -= 1;
    }

    Ok(RootNodeProxy {
      union_find: self,
      id: a_root_id,
    })
  }
}

impl<T: UnionFindData<Error = Infallible>> UnionFind<T> {
  /// Unions the two sets that a and b are in (noop if are already in the same
  /// set), returning the new set index of the two nodes.
  pub fn union(&mut self, a_id: usize, b_id: usize) -> RootNodeProxy<'_, T> {
    match self.try_union(a_id, b_id) {
      Ok(res) => res,
    }
  }
}

#[cfg(test)]
mod tests {
  use googletest::prelude::*;
  use std::{error::Error, fmt::Display};

  use crate::union_find::{RootNodeProxy, UnionFind, UnionFindData, UnionFindMergeError};

  #[gtest]
  fn test_basic() {
    let mut uf = UnionFind::new(10);

    for i in 0..10 {
      expect_eq!(uf.find(i).id, i);
    }

    uf.union(1, 3);
    uf.union(4, 5);
    uf.union(1, 5);

    expect_eq!(uf.const_find(1).id, uf.const_find(3).id);
    expect_eq!(uf.const_find(1).id, uf.const_find(4).id);
    expect_eq!(uf.const_find(1).id, uf.const_find(5).id);
    expect_eq!(uf.const_find(0).id, 0);
    expect_eq!(uf.const_find(2).id, 2);
    expect_eq!(uf.const_find(6).id, 6);
    expect_eq!(uf.const_find(7).id, 7);
    expect_eq!(uf.const_find(8).id, 8);
    expect_eq!(uf.const_find(9).id, 9);

    expect_eq!(uf.find(1).id, uf.find(3).id);
    expect_eq!(uf.find(1).id, uf.find(4).id);
    expect_eq!(uf.find(1).id, uf.find(5).id);
    expect_eq!(uf.find(0).id, 0);
    expect_eq!(uf.find(2).id, 2);
    expect_eq!(uf.find(6).id, 6);
    expect_eq!(uf.find(7).id, 7);
    expect_eq!(uf.find(8).id, 8);
    expect_eq!(uf.find(9).id, 9);
  }

  #[gtest]
  fn test_long_chain() {
    let mut uf = UnionFind::new(256);

    for i in 0..255 {
      uf.union(i, i + 1);
    }

    let root_id = uf.find(0).id;
    for i in 1..256 {
      expect_eq!(uf.find(i).id, root_id);
    }
  }

  #[gtest]
  fn test_fallible_data() {
    #[derive(Debug)]
    enum MyError {
      Overflow,
    }
    impl Display for MyError {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
      }
    }
    impl Error for MyError {}

    #[derive(Debug, PartialEq, Eq)]
    struct Data(u8);
    impl UnionFindData for Data {
      type Error = MyError;
      fn merge(
        &mut self,
        other: Self,
      ) -> std::result::Result<(), UnionFindMergeError<Self, MyError>> {
        match self.0.checked_add(other.0) {
          Some(sum) => {
            self.0 = sum;
            Ok(())
          }
          None => Err(UnionFindMergeError::new(MyError::Overflow, other)),
        }
      }
    }

    let mut uf = UnionFind::new_with_data([Data(10), Data(30), Data(255)]);

    expect_eq!(uf.find(0).id(), 0);
    expect_eq!(uf.find(0).data(), &Data(10));
    expect_eq!(uf.find(1).id(), 1);
    expect_eq!(uf.find(1).data(), &Data(30));
    expect_eq!(uf.find(2).id(), 2);
    expect_eq!(uf.find(2).data(), &Data(255));

    expect_that!(
      uf.try_union(0, 1),
      ok(property!(&RootNodeProxy::<Data>.data(), eq(&Data(40))))
    );
    expect_that!(
      uf.find(0),
      property!(&RootNodeProxy::<Data>.data(), eq(&Data(40)))
    );
    expect_that!(
      uf.find(1),
      property!(&RootNodeProxy::<Data>.data(), eq(&Data(40)))
    );

    expect_that!(uf.try_union(0, 2), err(pat!(MyError::Overflow)));
    // This should leave the UF unchanged.
    expect_that!(
      uf.find(0),
      property!(&RootNodeProxy::<Data>.data(), eq(&Data(40)))
    );
    expect_that!(
      uf.find(1),
      property!(&RootNodeProxy::<Data>.data(), eq(&Data(40)))
    );
    expect_that!(
      uf.find(2),
      property!(&RootNodeProxy::<Data>.data(), eq(&Data(255)))
    );
  }

  #[gtest]
  fn test_add_set() {
    let mut uf: UnionFind<()> = UnionFind::new(0);
    let id1 = uf.add_set(()).id();
    let id2 = uf.add_set(()).id();

    expect_that!(uf.unique_sets(), eq(2));
    expect_that!(uf.try_union(id1, id2), ok(anything()));
    expect_that!(uf.unique_sets(), eq(1));
  }
}
