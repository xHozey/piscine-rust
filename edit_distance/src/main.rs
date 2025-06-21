use edit_distance::*;

fn main() {
    let source = "benedictcumberbatch";
    let target = "beneficialcucumbersnatch";

    println!(
        "It's necessary to make {} change(s) to {:?} to get {:?}",
        edit_distance(source, target),
        source,
        target
    );
}