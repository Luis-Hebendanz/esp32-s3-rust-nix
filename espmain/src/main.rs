use espmain::complex_example_func;
use petgraph::dot::Dot;
use petgraph::prelude::Graph;

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
    complex_example_func();
}
