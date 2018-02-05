# seating-rs
A seat chart generator for schools that sorts students with others they haven't sat by yet

## Usage

Run `seating-rs [PATH/TO/SRC/FILE]`, where the selected file is a data file configured as such:


`[Number of students to seat in groups, or pods]
\|
\[A "-" seperated list of arrangements, which are lists of people, sorted into the groups they were sat by.\]
\-
[repeat, etc.]
\-`

## Example source file


`3
\|
Joe
Jeremy
Sarah
Plummbett
Jenkins
Benington
\-
Benington
Jenkins
Sarah
Joe
Jeremy
Plummbett
\-`

## Installation
`cargo build --release`
`sudo cp target/release/seating-rs /usr/bin/seating-rs`


