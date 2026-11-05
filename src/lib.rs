#[derive(Debug, Default)]
pub struct FrameAllocator {
    total_frames: usize,
    allocated_frames: usize,
}

impl FrameAllocator {
    pub fn new(total: usize) -> Self { Self { total_frames: total, allocated_frames: 0 } }
    pub fn allocate_frame(&mut self) -> Option<usize> {
        if self.allocated_frames < self.total_frames {
            let frame = self.allocated_frames;
            self.allocated_frames += 1;
            Some(frame)
        } else {
            None
        }
    }
    pub fn free_frames_count(&self) -> usize { self.total_frames - self.allocated_frames }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_frame_allocation() {
        let mut alloc = FrameAllocator::new(10);
        assert_eq!(alloc.allocate_frame(), Some(0));
        assert_eq!(alloc.free_frames_count(), 9);
    }
}
