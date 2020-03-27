extern crate clique;


use clique::{Edge, solve_clique, solve_clique_wrap};


fn main() {
    // Defining the graph
    let graph: Vec<Edge> = vec![
        Edge(0, 3), Edge(0, 4), Edge(0, 7),
        Edge(1, 2), Edge(1, 4), Edge(1, 5), Edge(1, 7),
        Edge(2, 3), Edge(2, 8),
        Edge(3, 5), Edge(3, 8),
        Edge(4, 5), Edge(4, 6), Edge(4, 7), Edge(4, 8),
        Edge(5, 7),
        Edge(6, 7),
        Edge(7, 8),
    ];
    println!("{:?}", graph);

    // Calculating clique with solve_clique
    let clique = solve_clique(&graph);
    println!("{:?}", clique);

    // Defining the graph as array of integers
    let graph_arr = [0, 3, 0, 4, 0, 7, 1, 2, 1, 4, 1, 5, 1, 7, 2, 3, 2, 8, 3, 5, 3, 8, 4, 5, 4, 6, 4, 7, 4, 8, 5, 7, 6, 7, 7, 8];
    let mut clique_arr: [u32; 9] = [0; 9];

    // Calculating clique with solve_clique_wrap
    let clique_size = solve_clique_wrap(graph_arr.len() as u32, &graph_arr, &mut clique_arr);
    println!("{:?}", clique_size);
    println!("{:?}", clique_arr);
}
