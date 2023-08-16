use crate::{bar::Bar, Algorithm, AlgorithmState, List};

pub struct BogoSort {}
impl Algorithm for BogoSort {
    type Item = Bar;

    fn new() -> Self
    where
        Self: Sized,
    {
        BogoSort {}
    }

    fn tick(&mut self, l: &mut List<Self::Item>) -> AlgorithmState {
        if l.sorted() {
            AlgorithmState::Done
        } else {
            let (reads, writes) = (l.reads, l.writes);
            *l = List::shuffled(l.size);
            l.writes = l.size + writes;
            l.reads = reads;
            AlgorithmState::Busy
        }
    }

    fn reset(&mut self) {
        *self = Self::new();
    }

    fn name(&self) -> &'static str {
        "bogosort"
    }
}
