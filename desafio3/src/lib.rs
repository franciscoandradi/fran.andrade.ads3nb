use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::Dfs;

pub struct MyGraph {
    graph: Graph<&'static str, ()>,
    node1_index: NodeIndex,
}

impl MyGraph {
    pub fn new() -> Self {
        let mut graph = Graph::<&'static str, ()>::new();

        // to usando 1,2,3,4,5,6 só porque já estão sendo usados no test
        let n1 = graph.add_node("1");
        let n2 = graph.add_node("2");
        let n3 = graph.add_node("3");
        let n4 = graph.add_node("4");
        let n5 = graph.add_node("5");
        let n6 = graph.add_node("6");

        // conexao basica entre os numeros
        graph.add_edge(n1, n2, ());
        graph.add_edge(n2, n3, ());
        graph.add_edge(n3, n4, ());
        graph.add_edge(n4, n5, ());
        graph.add_edge(n5, n6, ());

        MyGraph {
            graph,
            node1_index: n1,
        }
    }

    pub fn dfs_from_node1(&self) -> Vec<&'static str> {
        let mut dfs = Dfs::new(&self.graph, self.node1_index);
        let mut visited = Vec::new();

        while let Some(nx) = dfs.next(&self.graph) {
            let node_data = self.graph[nx];
            visited.push(node_data);
        }

        visited
    }
}

// -----------------------------------------------------------
// TESTES
// -----------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_visits_all_nodes() {
        let g = MyGraph::new();
        let result = g.dfs_from_node1();

        let mut sorted_result = result.clone();
        sorted_result.sort();

        let mut expected = vec!["1", "2", "3", "4", "5", "6"];
        expected.sort();

        assert_eq!(sorted_result, expected, "Todos os nós devem ser visitados");
    }

    #[test]
    fn test_dfs_starts_at_node1() {
        let g = MyGraph::new();
        let result = g.dfs_from_node1();
        assert_eq!(result.first(), Some(&"1"), "DFS deve começar pelo nó 1");
    }
}