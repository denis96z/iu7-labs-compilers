use std::collections::{HashSet, VecDeque};

pub type Stack<T> = Vec<T>;
pub type Queue<T> = VecDeque<T>;
pub type Set<T> = HashSet<T>;

pub type Symbol = String;
pub type SymbolRef = &'static str;
