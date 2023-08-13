use crate::{Algorithm, AlgorithmState, List};
use crate::bar::Bar;

pub struct GnomeSort {
    gnome_pos: usize,
}

impl Algorithm for GnomeSort {
    type Item = Bar;
    const NAME: &'static str = "gnome sort";
    fn new() -> Self {
        GnomeSort { gnome_pos: 0 }
    }

    fn tick(&mut self, l: &mut List<Self::Item>) -> AlgorithmState {
        if self.gnome_pos == l.size {
            AlgorithmState::Done
        } else {
            l.deselect_all();
            if self.gnome_pos == 0 || l.select(self.gnome_pos) > l.select(self.gnome_pos - 1) {
                self.gnome_pos += 1;
            } else {
                l.swap(self.gnome_pos, self.gnome_pos - 1);
                self.gnome_pos -= 1;
            }
            AlgorithmState::Busy
        }
    }
}
