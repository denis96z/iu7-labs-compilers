use std::collections;

pub type Stack<T> = Vec<T>;
pub type Queue<T> = collections::VecDeque<T>;
pub type Set<T> = collections::HashSet<T>;

pub type Symbol = String;
pub type SymbolRef = &'static str;
