use mymacros::story;

fn main() {
    story!(
        go north.
        go east.
        take potion.
        drop potion.
        give potion to wizard.
        go west.
        go south.
    );
}
