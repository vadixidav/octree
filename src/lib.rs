mod morton;
mod tiered_morton;

pub use crate::tiered_morton::TieredMorton;

pub trait Tiered {
    type Reduced: Tiered<Reduced = Self::Reduced>;

    fn reduce(lhs: &Self, rhs: &Self) -> Self::Reduced;
}
