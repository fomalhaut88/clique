use std::collections::{HashMap, HashSet};


#[derive(Debug, Clone, Copy)]
pub struct Edge(pub u32, pub u32);


#[no_mangle]
pub fn solve_clique_wrap(size: u32, graph: &[u32; 1000000], clique: &mut [u32; 1000]) -> u32 {
    // Building graph as Vec<Edge>
    let mut graph_vec: Vec<Edge> = Vec::new();
    for i in (0..size).step_by(2) {
        graph_vec.push(Edge(graph[i as usize], graph[i as usize + 1]));
    }

    // Applying solve_clique
    let clique_set = solve_clique(&graph_vec);

    // Filling clique from returned clique_set (as HashSet<u32>)
    for (i, vert) in clique_set.iter().enumerate() {
        clique[i] = *vert;
    }

    // Returning the size of the clique found
    clique_set.len() as u32
}


pub fn solve_clique(graph: &Vec<Edge>) -> HashSet<u32> {
    // Calculating how many edges each vertex has
    let stat = {
        let mut stat = HashMap::new();
        for edge in graph.iter() {
            let e0 = stat.entry(edge.0).or_insert(0); *e0 += 1;
            let e1 = stat.entry(edge.1).or_insert(0); *e1 += 1;
        }
        stat
    };

    // Searching for the vertex with the minimum number of edges
    let vert_min = {
        let mut count_min = stat.values().next().unwrap() + 1;
        let mut vert_min = None;
        for (vert, count) in &stat {
            if *count < count_min {
                count_min = *count;
                vert_min = Some(*vert);
            }
        }
        vert_min.unwrap()
    };

    // The graph is clique if E = V * (V - 1) / 2,
    // where E is the number of edges of the graph (graph.len()),
    // and V is the number of vertices,
    // that is equal to (count_min + 1) for clique
    let count_min = stat[&vert_min];
    if count_min * (count_min + 1) == 2 * (graph.len() as u32) {
        return {
            let mut vertices: HashSet<u32> = HashSet::new();
            for edge in graph.iter() {
                vertices.insert(edge.0);
                vertices.insert(edge.1);
            }
            vertices
        };
    }

    // Building major subgraph (throwing vert_min away from the graph)
    let subgraph_major = {
        let mut subgraph = Vec::new();
        for edge in graph.iter() {
            if edge.0 != vert_min && edge.1 != vert_min {
                subgraph.push(*edge);
            }
        }
        subgraph
    };

    // Calculating the clique for the major subgraph
    let clique_major = solve_clique(&subgraph_major);

    // Searching for the vertices of the minor subgraph
    // (that are connected to vert_min)
    let subgraph_minor_vertices = {
        let mut vertices: HashSet<u32> = HashSet::new();
        for edge in graph.iter() {
            if edge.0 == vert_min {
                vertices.insert(edge.1);
            }
            if edge.1 == vert_min {
                vertices.insert(edge.0);
            }
        }
        vertices
    };

    // Optimization: if the clique of the major subgraph
    // if bigger than the number of vertices in the minor subgraph,
    // there's no need to search for clique of the minor sibgraph,
    // we can return clique_major as the result
    if clique_major.len() > subgraph_minor_vertices.len() {
        return clique_major;
    }

    // Building minor subgraph (from the vertices that are connected to vert_min)
    let subgraph_minor = {
        let mut subgraph = Vec::new();
        for edge in graph.iter() {
            if subgraph_minor_vertices.contains(&edge.0) && subgraph_minor_vertices.contains(&edge.1) {
                subgraph.push(*edge);
            }
        }
        subgraph
    };

    // Calculating the clique of the minor subgraph (adding vert_min in the end)
    let clique_minor = {
        let mut clique = solve_clique(&subgraph_minor);
        clique.insert(vert_min);
        clique
    };

    // Returning clique_major or clique_minor depending on which size is bigger
    return if clique_major.len() >= clique_minor.len() { clique_major } else { clique_minor }
}
