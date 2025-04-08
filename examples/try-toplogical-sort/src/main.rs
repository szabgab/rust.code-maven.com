use topological_sort::TopologicalSort;

fn main() {
    independent_things();
    pop_independent_things();
    simple_dependencies();
    complex_dependencies();
    circular_dependencies();
    pop_circular_dependencies();

    many_circular_dependencies();
}

// First < Second < Third
fn simple_dependencies() {
    let mut ts = TopologicalSort::<&str>::new();
    ts.add_dependency("First", "Second");
    ts.add_dependency("Second", "Third");
    assert_eq!(ts.len(), 3);

    // Fetch the elements that don't depend on anything
    let all = ts.pop_all();
    assert_eq!(all.len(), 1);
    assert_eq!(all[0], "First");

    let all = ts.pop_all();
    assert_eq!(all.len(), 1);
    assert_eq!(all[0], "Second");

    let all = ts.pop_all();
    assert_eq!(all.len(), 1);
    assert_eq!(all[0], "Third");

    let all = ts.pop_all();
    assert_eq!(all.len(), 0);
}

fn independent_things() {
    let mut ts = TopologicalSort::<&str>::new();
    assert_eq!(ts.len(), 0);
    assert!(ts.is_empty());

    ts.insert("A");
    ts.insert("B");
    ts.insert("C");
    assert_eq!(ts.len(), 3);
    // println!("{:?}", ts);


    // The order of insertion does not matter, we get the items in arbitrary order
    let all = ts.pop_all();
    assert_eq!(all.len(), 3);
    for thing in ["A", "B", "C"] {
        assert!(all.contains(&thing));
    }

    assert!(ts.is_empty());
}

fn pop_independent_things() {
    let mut ts = TopologicalSort::<&str>::new();
    assert_eq!(ts.len(), 0);
    assert!(ts.is_empty());

    ts.insert("A");
    ts.insert("B");
    ts.insert("C");
    assert_eq!(ts.len(), 3);

    // The order of insertion does not matter, we get the items in arbitrary order
    let mut all = vec!["A", "B", "C"];

    for _ in 0..3 {
        let elem = ts.pop();
        let elem = elem.unwrap();
        assert!(all.contains(&elem));
        let index = all.iter().position(|x| *x == elem).unwrap();
        all.remove(index);
    }
    assert_eq!(all.len(), 0);
    assert_eq!(ts.len(), 0);
}

// First A < Second
// First B < Second
// Second < Third A
// Second < Third B
fn complex_dependencies() {
    let mut ts = TopologicalSort::<&str>::new();
    ts.add_dependency("First A", "Second");
    ts.add_dependency("First B", "Second");
    ts.add_dependency("Second", "Third A");
    ts.add_dependency("Second", "Third B");
    assert_eq!(ts.len(), 5);

    let all = ts.pop_all();
    assert_eq!(all.len(), 2);
    assert!(all.contains(&"First A"));
    assert!(all.contains(&"First B"));

    let all = ts.pop_all();
    assert_eq!(all.len(), 1);
    assert!(all.contains(&"Second"));

    let all = ts.pop_all();
    assert_eq!(all.len(), 2);
    assert!(all.contains(&"Third A"));
    assert!(all.contains(&"Third B"));
}

// Chicken <-> Egg
fn circular_dependencies() {
    let mut ts = TopologicalSort::<&str>::new();
    ts.add_dependency("Chicken", "Egg");
    ts.add_dependency("Egg", "Chicken");
    // println!("{:?}", ts);

    // if pop_all returned an empty vector and there are still elements in the TS
    // that means the dependencies are circular
    let all = ts.pop_all();
    assert_eq!(all.len(), 0);
    assert_eq!(ts.len(), 2);
}

// Chicken <-> Egg
// Shakshuka < Egg
// Chicken < Qqrq
fn pop_circular_dependencies() {
    let mut ts = TopologicalSort::<&str>::new();
    ts.add_dependency("Chicken", "Egg");
    ts.add_dependency("Egg", "Chicken");
    ts.add_dependency("Shakshuka", "Egg");
    ts.add_dependency("Chicken", "Qqrq");
    // println!("{:?}", ts);

    let elem = ts.pop();
    assert_eq!(elem, Some("Shakshuka"));

    let elem = ts.pop();
    assert_eq!(elem, None);
}


fn many_circular_dependencies() {
    let mut ts = TopologicalSort::<&str>::new();
    ts.add_dependency("Chicken", "Egg");
    ts.add_dependency("Egg", "Chicken");

    ts.add_dependency("Egg", "Omlet");
    // ts.add_dependency("A", "Egg");
    // ts.add_dependency("B", "Omlet");
    // ts.add_dependency("A", "B");
    // ts.add_dependency("B", "A");


    println!("{:?}", ts);
    assert_eq!(ts.pop(), None);
}