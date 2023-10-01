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

    // FNV-1a hash taking bytes from bitboards
    // We don't need to hash to-play since positions are unique per player
    // If they weren't, players could make an infinite loop
    fn hash(node: &BreakthroughNode) -> u64 {
        const H: u64 = 14695981039346656037;
        const P: u64 = 1099511628211;
        let mut hash = H;
        for i in 0..8 {
            let byte = (node.bitboard_white >> (8 * i)) & 0xff;
            hash = (hash ^ byte).wrapping_mul(P);
            let byte = (node.bitboard_black >> (8 * i)) & 0xff;
            hash = (hash ^ byte).wrapping_mul(P);
        }
        hash
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
                self.table[index] = Some(entry);
                self.occupied += 1;
            }
            Some(value) => {
                if value.0 == entry.0 {
                    self.table[index] = Some(entry);
                } else {
                    self.table[index] = Some(entry);
                    self.collisions += 1;
                }
            }
        }
    }

    pub fn stats(&self) -> (usize, usize, usize) {
        (self.capacity, self.occupied, self.collisions)
    }
}
