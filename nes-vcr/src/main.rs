use petgraph::dot::Dot;
use petgraph::prelude::Graph;
use anyhow::anyhow;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    let mut arr = vec![1, 2, 3];
    arr.push(4);
    log::info!("Hello, world! {:?}", arr);
    let mut graph = Graph::<&str, u32>::new();
    let origin = graph.add_node("Denver");
    let destination_1 = graph.add_node("San Diego");
    let destination_2 = graph.add_node("New York");

    graph.extend_with_edges([(origin, destination_1, 250), (origin, destination_2, 1099)]);

    println!("{}", Dot::new(&graph));
    foo().unwrap();
}

fn foo() -> Result<(), anyhow::Error> {
    println!("foor1");
    foo2()
}
fn foo2() -> Result<(), anyhow::Error> {
    println!("foor2");
    Err::<_, anyhow::Error>(anyhow!("Test error"))
}

// Add this block to your code
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use petgraph::algo::{dijkstra, min_spanning_tree};

//     #[test]
//     fn test_graph() {
//         // Create the same graph as in the main function
//         let mut graph = Graph::<&str, u32>::new();
//         let origin = graph.add_node("Denver");
//         let destination_1 = graph.add_node("San Diego");
//         let destination_2 = graph.add_node("New York");

//         graph.extend_with_edges([(origin, destination_1, 250), (origin, destination_2, 1099)]);

//         // Test the number of nodes and edges
//         assert_eq!(graph.node_count(), 3);
//         assert_eq!(graph.edge_count(), 2);

//         // Test the neighbors of a node
//         assert_eq!(graph.neighbors(origin).collect::<Vec<_>>(), vec![destination_1, destination_2]);

//         // Test the shortest path from origin to destination_2
//         let node_map = dijkstra(&graph, origin, Some(destination_2), |e| *e.weight());
//         assert_eq!(&1099, node_map.get(&destination_2).unwrap());

//         // Test the minimum spanning tree of the graph
//         let mst = Graph::<_, _>::from_elements(min_spanning_tree(&graph));
//         assert_eq!(mst.raw_edges().len(), 2);
//         assert_eq!(mst.raw_edges()[0].weight, 250);
//         assert_eq!(mst.raw_edges()[1].weight, 1099);
//     }
// }