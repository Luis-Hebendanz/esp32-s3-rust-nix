use petgraph::{prelude::Graph, dot::Dot};


pub fn complex_example_func() {
    let mut arr = vec![1, 2, 3];
    arr.push(4);
    log::info!("Hello, world! {:?}", arr);
    let mut graph = Graph::<&str, u32>::new();
    let origin = graph.add_node("Denver");
    let destination_1 = graph.add_node("San Diego");
    let destination_2 = graph.add_node("New York");

    graph.extend_with_edges([(origin, destination_1, 250), (origin, destination_2, 1099)]);

    println!("{}", Dot::new(&graph));
}
