#[derive(Debug)]
pub enum Error {
    IllegalSet(usize, u8),
    SegmentationFault(u16),
}
