<img src="orst.png" width=300>

# orst - a simple sorting algorithm visualiser

orst is a super simple program which visualises different sorting algorithms; nothing original, just something a little fun! (⌒‿⌒)

the name is just the letters of "sort" sorted alphabetically.

## usage

simply run orst from the command line. it defaults to bubble sorting 40 bars.

## adding algorithms

adding a sorting algorithm is super easy! all you have to do is define a new struct containing all the needed local state for the algorithm (e.g. the location of pointers, the biggest element found so far, ...). then you need to implement the `Algorithm` trait on that struct. `Algorithm` has an associated type `Item`, which needs to implement `ListItem`. any type that implements `ListItem` needs to be `Ord` (it needs to be able to compared to other instances), `Clone` and `From<usize>` (i.e. there needs to be a way to take a `usize` and convert it into an instance of the type). orst provides you with `Bar`, a type that already implements `ListItem`.

implementing `Algorithm` means supplying a few things:
* providing a `NAME`, a `&'static str` containing the name of the algorithm,
* providing a method called `new()` which returns a fresh instance of the local state,
* most importantly, providing the `tick(&mut self, l: &mut List) -> AlgorithmState` method.  
  `tick()` is essentialy "one step" of your algorithm. this could be comparing two values, advancing a pointer, swapping two elements, etcetera. it returns an `AlgorithmState`, an enum with two possible values: `Busy` and `Done`. if the algorithm returns `Done`, orst will await user input (specifically the right arrow key), reshuffle the array and start anew.