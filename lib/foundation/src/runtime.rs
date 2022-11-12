use core::cmp::Ordering;

#[derive(Clone, Copy, Debug)]
#[repr(isize)]
pub enum NSComparisonResult {
    Ascending = -1,
    Same = 0,
    Descending = 1,
}

impl From<NSComparisonResult> for Ordering {
    fn from(result: NSComparisonResult) -> Self {
        match result {
            NSComparisonResult::Ascending => Self::Less,
            NSComparisonResult::Same => Self::Equal,
            NSComparisonResult::Descending => Self::Greater,
        }
    }
}
