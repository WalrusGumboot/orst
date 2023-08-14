use crate::bar::Bar;
use crate::{Algorithm, AlgorithmState, List, NUM_ELEM_CELL};

#[derive(Debug)]
enum InternalState {
    RunningCombPass,
    AdjustingGapSize,
}

#[derive(Debug)]
enum ReturnValue {
    False,
    TrueNextRun,
    DefinitelyTrue
}

#[derive(Debug)]
pub struct CombSort {
    shrink_factor: f64,
    gap_size: usize,
    sorted: ReturnValue,

    comb_pos: usize,
    state: InternalState,
}

impl Algorithm for CombSort {
    type Item = Bar;

    fn new() -> Self
    where
        Self: Sized,
    {
        CombSort {
            shrink_factor: 1.3,
            gap_size: unsafe { *NUM_ELEM_CELL.get().unwrap() } - 1,
            sorted: ReturnValue::False,
            comb_pos: 0,
            state: InternalState::RunningCombPass,
        }
    }

    fn tick(&mut self, l: &mut List<Self::Item>) -> AlgorithmState {
        l.deselect_all();
        if matches!(self.sorted, ReturnValue::DefinitelyTrue) {
            AlgorithmState::Done
        } else {
            match self.state {
                InternalState::AdjustingGapSize => {
                    let ideal_gap_size = self.gap_size as f64 / self.shrink_factor;
                    if ideal_gap_size <= 1.0 {
                        self.gap_size = 1;
                        self.sorted = ReturnValue::TrueNextRun;
                    } else {
                        self.gap_size = ideal_gap_size.floor() as usize;
                    }

                    self.comb_pos = 0;
                    self.state = InternalState::RunningCombPass;
                }
                InternalState::RunningCombPass => {
                    if self.comb_pos + self.gap_size < l.size {
                        if l.select(self.comb_pos) > l.select(self.comb_pos + self.gap_size) {
                            l.swap(self.comb_pos, self.comb_pos + self.gap_size);
                            self.sorted = ReturnValue::False
                        }
                        
                        self.comb_pos += 1;
                    } else {
                        self.state = InternalState::AdjustingGapSize;

                        if matches!(self.sorted, ReturnValue::TrueNextRun) {
                            self.sorted = ReturnValue::DefinitelyTrue
                        }
                    }
                }
            }

            AlgorithmState::Busy
        }
    }

    fn reset(&mut self) {
        *self = Self::new();
    }

    fn name(&self) -> &'static str {
        "comb sort"
    }
}
