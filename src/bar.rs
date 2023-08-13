use sdl2::pixels::Color;
use std::cmp::Ordering::{self, *};

use crate::{util::lerp_color, ListItem, NUM_ELEM_CELL};

pub const LEFT_COLOR: Color = Color::RGB(222, 100, 91);
pub const RIGHT_COLOUR: Color = Color::RGB(126, 160, 224);

#[derive(Clone, PartialEq, Eq)]
pub struct Bar {
    pub value: usize,
    pub colour: Color,
    pub selected: bool,
}

impl From<usize> for Bar {
    fn from(value: usize) -> Self {
        Bar {
            value,
            colour: lerp_color(
                LEFT_COLOR,
                RIGHT_COLOUR,
                value as f64 / unsafe { *NUM_ELEM_CELL.get().unwrap() } as f64,
            ),
            selected: false,
        }
    }
}

impl Default for Bar {
    fn default() -> Self {
        Bar {
            value: 0,
            colour: Color::BLACK,
            selected: false,
        }
    }
}

impl PartialOrd for Bar {
    fn lt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Less))
    }

    fn le(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Less | Equal))
    }

    fn gt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Greater))
    }

    fn ge(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Greater | Equal))
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Bar {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl ListItem for Bar {
    fn select(&mut self) {
        self.selected = true;
    }

    fn deselect(&mut self) {
        self.selected = false;
    }
}
