use clap::ValueEnum;

// exchange sorts
pub mod gnome;
pub mod bubble;
pub mod optimised_bubble;

#[derive(Clone, ValueEnum)]
pub enum AlgorithmType {
    Gnome,
    Bubble,
    OptimisedBubble
}