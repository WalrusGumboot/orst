use crate::bar::Bar;
use crate::{Algorithm, AlgorithmState, List};

pub struct GnomeSort {
    gnome_pos: usize,
}

impl Algorithm for GnomeSort {
    type Item = Bar;
    fn name(&self) -> &'static str {
        "gnome sort"
    }
    fn new() -> Self {
        GnomeSort { gnome_pos: 0 }
    }

    fn tick(&mut self, l: &mut List<Self::Item>) -> AlgorithmState {
        l.deselect_all();
        if self.gnome_pos == l.size {
            AlgorithmState::Done
        } else {
            if self.gnome_pos == 0 || l.select(self.gnome_pos) > l.select(self.gnome_pos - 1) {
                self.gnome_pos += 1;
            } else {
                l.swap(self.gnome_pos, self.gnome_pos - 1);
                self.gnome_pos -= 1;
            }
            AlgorithmState::Busy
        }
    }

    fn reset(&mut self) {
        *self = Self::new();
    }
}
