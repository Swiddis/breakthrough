use crate::core::{node8::BreakthroughNode, Evaluation};

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

    // FNV-1a hash using bitboards instead of bytes
    fn hash(node: &BreakthroughNode) -> u64 {
        const H: u64 = 0xcbf29ce484222325;
        const P: u64 = 0x100000001b3;
        // Don't need to hash to_play since positions are unique
        // If they weren't, the players could make an infinite loop
        ((H ^ node.bitboard_white).wrapping_mul(P) ^ node.bitboard_black).wrapping_mul(P)
    }

    fn get_with_index<'a>(&'a self, node: &BreakthroughNode) -> (usize, &'a Option<Entry>) {
        let index: usize = (Self::hash(node) as usize) % self.capacity;
        (index, self.table.get(index).expect("index should be in bounds"))
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
