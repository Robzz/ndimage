//! Contains the definition of neighborhood shapes and neighborhood iterators.

/// Trait for types describing neighborhood shapes. Broadly speaking, a Neighborhood is defined by its origin and a set
/// of pixels whose position is relative to the origin.
pub trait Neighborhood {
    /// Return the origin of the neighborhood.
    fn origin(&self) -> (u32, u32);
}

/// Rectangular neighborhood with a specified origin.
#[allow(dead_code)]
pub struct RectNeighborhood {
    size: (u32, u32),
    origin: (u32, u32)
}

impl Neighborhood for RectNeighborhood {
    fn origin(&self) -> (u32, u32) {
        self.origin
    }
}

impl RectNeighborhood {
    /// Create a new `RectNeighborhood`. The origin must be inside of the neighborhood and size must be non zero.
    pub fn new(size: (u32, u32), origin: (u32, u32)) -> Option<RectNeighborhood> {
        if origin.0 <= size.0 && origin.1 < size.1 && size.0 > 0 && size.1 > 0 {
            Some(RectNeighborhood { size, origin })
        } else {
            None
        }
    }
}

// TODO
// Iterator over a rectangular image region.
//pub struct RectNeighborhoodIter {
//}

#[cfg(test)]
mod tests {
    use core::RectNeighborhood;

    #[test]
    fn test_new_rect_neighborhood() {
        assert!(RectNeighborhood::new((100, 100), (50, 50)).is_some());
        assert!(RectNeighborhood::new((100, 100), (99, 99)).is_some());
        assert!(RectNeighborhood::new((100, 100), (0, 0)).is_some());
        assert!(RectNeighborhood::new((100, 100), (100, 100)).is_none());
        assert!(RectNeighborhood::new((0, 100), (0, 0)).is_none());
        assert!(RectNeighborhood::new((100, 0), (0, 0)).is_none());
        assert!(RectNeighborhood::new((0, 0), (0, 0)).is_none());
    }
}
