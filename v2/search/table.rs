use crate::{
    core::{node8::BreakthroughNode, Evaluation},
    search::zobrist::zobrist_keys,
};

type Entry = (BreakthroughNode, u32, Evaluation);

pub struct TranspositionTable {
    capacity: usize,
    collisions: usize,
    occupied: usize,
    table: Vec<Option<Entry>>,
}

impl TranspositionTable {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            collisions: 0,
            occupied: 0,
            table: vec![None; capacity],
        }
    }

    // FNV-1a hash taking bytes from bitboards
    // We don't need to hash to-play since positions are unique per player
    // If they weren't, players could make an infinite loop
    fn hash(node: &BreakthroughNode) -> u64 {
        let keys = zobrist_keys();
        (0..64)
            .map(|i| {
                match (
                    node.bitboard_white & (1 << i),
                    node.bitboard_black & (1 << i),
                ) {
                    (0, 0) => keys[i],
                    (0, _) => keys[64 + i],
                    (_, _) => keys[128 + i],
                }
            })
            .reduce(|a, b| a ^ b)
            .unwrap_or(0)
    }

    fn get_with_index<'a>(&'a self, node: &BreakthroughNode) -> (usize, &'a Option<Entry>) {
        let index: usize = (Self::hash(node) as usize) % self.capacity;
        (
            index,
            self.table.get(index).expect("index should be in bounds"),
        )
    }

    pub fn get(&self, node: &BreakthroughNode, depth: u32) -> Option<&Entry> {
        if self.capacity == 0 {
            return None;
        }

        let result = self.get_with_index(node).1;
        match result {
            None => None,
            Some(entry) => match &entry.0 == node && entry.1 >= depth {
                false => None,
                true => Some(entry),
            },
        }
    }

    pub fn put(&mut self, entry: Entry) {
        if self.capacity == 0 {
            return;
        }

        let (index, current) = self.get_with_index(&entry.0);
        match current {
            None => {
                self.table.insert(index, Some(entry));
                self.occupied += 1;
            }
            Some(value) => {
                if value.0 == entry.0 {
                    self.table.insert(index, Some(entry));
                } else {
                    self.table.insert(index, Some(entry));
                    self.collisions += 1;
                }
            }
        }
    }

    pub fn stats(&self) -> (usize, usize, usize) {
        (self.capacity, self.occupied, self.collisions)
    }
}
