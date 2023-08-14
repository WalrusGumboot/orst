use clap::ValueEnum;

// exchange sorts
pub mod gnome;
pub mod bubble;
pub mod optimised_bubble;

pub mod comb;

// insertion sorts
pub mod insertion;

#[derive(Clone, ValueEnum)]
pub enum AlgorithmType {
    Gnome,
    Bubble,
    OptimisedBubble,
    Comb,
    Insertion,
}