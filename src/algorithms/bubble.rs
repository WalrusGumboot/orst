use crate::{Algorithm, AlgorithmState, List};
use crate::bar::Bar;

pub struct BubbleSort {
    has_swapped_this_iteration: bool,
    current_position: usize,
}

impl Algorithm for BubbleSort {
    type Item = Bar;
    const NAME: &'static str = "bubble sort";
    fn new() -> Self {
        BubbleSort {
            has_swapped_this_iteration: false,
            current_position: 1,
        }
    }

    fn tick(&mut self, l: &mut List<Self::Item>) -> AlgorithmState {
        // reset all selections
        l.deselect_all();

        if !self.has_swapped_this_iteration && self.current_position == l.size {
            AlgorithmState::Done
        } else {
            if self.current_position == l.size {
                self.has_swapped_this_iteration = false;
                self.current_position = 1;
            }

            if l.select(self.current_position - 1) > l.select(self.current_position) {
                self.has_swapped_this_iteration = true;
                l.swap(self.current_position, self.current_position - 1);
            }

            self.current_position += 1;
            AlgorithmState::Busy
        }
    }
}
