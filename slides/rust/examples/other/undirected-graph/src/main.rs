#[allow(dead_code)]
#[derive(Debug)]
struct Node {
    // Vertex, Point
    name: String,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Edge<'a> {
    // Arcs
    a: &'a Node,
    b: &'a Node,
}

fn main() {
    let kneiphof = Node {
        name: String::from("Kneiphof"),
    };
    let lomse = Node {
        name: String::from("Lomse"),
    };
    let left = Node {
        name: String::from("Left bank"),
    };
    let right = Node {
        name: String::from("Right bank"),
    };

    let mut edges: Vec<Edge> = vec![];
    edges.push(Edge {
        a: &kneiphof,
        b: &lomse,
    });

    edges.push(Edge {
        a: &left,
        b: &lomse,
    });

    edges.push(Edge {
        a: &left,
        b: &lomse,
    });

    edges.push(Edge {
        a: &left,
        b: &kneiphof,
    });

    println!("{:#?}", edges);
}
