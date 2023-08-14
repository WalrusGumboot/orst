use crate::{bar::Bar, Algorithm, AlgorithmState, List};

pub struct InsertionSort {
    current_pos: usize,
    swapping_pointer: usize,
}

impl Algorithm for InsertionSort {
    type Item = Bar;

    fn new() -> Self
    where
        Self: Sized,
    {
        InsertionSort {
            current_pos: 1,
            swapping_pointer: 1,
        }
    }

    fn tick(&mut self, l: &mut List<Self::Item>) -> AlgorithmState {
        l.deselect_all();
        if self.current_pos >= l.size {
            AlgorithmState::Done
        } else {
            if self.swapping_pointer > 0
                && l.select(self.swapping_pointer - 1) > l.select(self.swapping_pointer)
            {
                l.swap(self.swapping_pointer - 1, self.swapping_pointer);
                self.swapping_pointer -= 1;
            } else {
                self.current_pos += 1;
                self.swapping_pointer = self.current_pos;
            }

            AlgorithmState::Busy
        }
    }

    fn reset(&mut self) {
        *self = Self::new();
    }

    fn name(&self) -> &'static str {
        "insertion sort"
    }
}
