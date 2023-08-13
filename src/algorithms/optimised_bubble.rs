use crate::{Algorithm, AlgorithmState, List, NUM_ELEM};
use crate::bar::Bar;

pub struct OptimisedBubbleSort {
    has_swapped_this_iteration: bool,
    current_position: usize,
    already_ok: usize,
}

impl Algorithm for OptimisedBubbleSort {
    type Item = Bar;
    const NAME: &'static str = "optimised bubble sort";
    fn new() -> Self {
        OptimisedBubbleSort {
            has_swapped_this_iteration: false,
            current_position: 1,
            already_ok: NUM_ELEM
        }
    }

    fn tick(&mut self, l: &mut List<Self::Item>) -> AlgorithmState {
        // reset all selections
        l.deselect_all();

        if !self.has_swapped_this_iteration && self.current_position == l.size {
            AlgorithmState::Done
        } else {
            if self.current_position == self.already_ok {
                self.has_swapped_this_iteration = false;
                self.current_position = 1;
                self.already_ok -= 1;
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
