use std::cell::OnceCell;

pub static mut NUM_ELEM_CELL: OnceCell<usize> = OnceCell::new();

pub struct List<T>
where
    T: Ord + Clone + From<usize>,
{
    content: Vec<T>,
    selected_indices: (Option<usize>, Option<usize>),

    pub size: usize,
    pub reads: usize,
    pub writes: usize,
}

pub trait ListItem
where
    Self: Ord + Clone + From<usize>,
{
    fn select(&mut self);
    fn deselect(&mut self);
}

impl<T: ListItem> List<T> {
    pub fn new(size: usize) -> Self {
        List {
            size,
            content: (0..size).map(|i| T::from(i)).collect(),
            selected_indices: (None, None),
            reads: 0,
            writes: 0,
        }
    }

    pub fn reversed(size: usize) -> Self {
        List {
            size,
            content: (0..size).map(|i| T::from(size - i)).collect(),
            selected_indices: (None, None),
            reads: 0,
            writes: 0,
        }
    }

    pub fn shuffled(size: usize) -> Self {
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        let mut base = Self::new(size);
        base.content.shuffle(&mut thread_rng());

        base
    }

    pub fn get(&self, i: usize) -> T {
        self.content[i].clone()
    }

    pub fn set(&mut self, i: usize, new: T) {
        self.content[i] = new;
    }

    pub fn swap(&mut self, i: usize, j: usize) {
        let temp = self.get(i);
        self.set(i, self.get(j));
        self.set(j, temp);

        self.writes += 2;
    }

    pub fn deselect_all(&mut self) {
        if let (Some(a), Some(b)) = self.selected_indices {
            self.deselect(a);
            self.deselect(b);
            self.selected_indices = (None, None);
        }
    }

    pub fn select(&mut self, i: usize) -> T {
        let mut val = self.get(i);
        val.select();
        self.set(i, val.clone());

        match self.selected_indices {
            (None, None) => {
                self.selected_indices.0 = Some(i);
            }
            (Some(_), None) => {
                self.selected_indices.1 = Some(i);
            }
            _ => unreachable!(),
        }

        self.reads += 1;

        val
    }

    pub fn deselect(&mut self, i: usize) {
        let mut val = self.get(i);
        val.deselect();
        self.set(i, val.clone());
    }
}

pub enum AlgorithmState {
    Busy,
    Done,
}

pub trait Algorithm
where
    Self::Item: ListItem,
{
    type Item;
    fn new() -> Self
    where
        Self: Sized;
    fn tick(&mut self, l: &mut List<Self::Item>) -> AlgorithmState;
    fn reset(&mut self);
    fn name(&self) -> &'static str;
}

mod algorithms;
mod bar;
mod util;

////////////////////////////////////////////
//            BAR RELATED CODE            //
////////////////////////////////////////////
