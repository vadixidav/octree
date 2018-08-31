use crate::morton::Morton;

use crate::Tiered;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TieredMorton<T: Tiered> {
    leaves: HashMap<Morton, T>,
    parents: HashMap<Morton, T::Reduced>,
}

impl<T> Default for TieredMorton<T>
where
    T: Tiered,
{
    fn default() -> Self {
        TieredMorton {
            leaves: HashMap::default(),
            parents: HashMap::default(),
        }
    }
}

impl<T> TieredMorton<T>
where
    T: Tiered,
{
    pub fn new() -> Self {
        TieredMorton::default()
    }
    pub fn leaves(&self) -> impl Iterator<Item = (&Morton, &T)> {
        self.leaves.iter()
    }
}
