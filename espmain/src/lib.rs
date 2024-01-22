use petgraph::algo::dijkstra;
use petgraph::prelude::Graph;


pub fn complex_example_func() {
    // Create the same graph as in the main function
    let mut graph = Graph::<&str, u32>::new();
    let origin = graph.add_node("Denver");
    let destination_1 = graph.add_node("San Diego");
    let destination_2 = graph.add_node("New York");

    graph.extend_with_edges([(origin, destination_1, 250), (origin, destination_2, 1099)]);

    // Test the number of nodes and edges
    assert_eq!(graph.node_count(), 3);
    assert_eq!(graph.edge_count(), 2);

    // Test the neighbors of a node
    assert_eq!(
        graph.neighbors(origin).collect::<Vec<_>>(),
        vec![destination_1, destination_2]
    );

    // Test the shortest path from origin to destination_2
    let node_map = dijkstra(&graph, origin, Some(destination_2), |e| *e.weight());
    assert_eq!(&1099, node_map.get(&destination_2).unwrap());
}
