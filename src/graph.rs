use std::collections::HashSet;

pub type NodeIdentifier = uint;

pub struct Edge<V> {
    data: V,
    dest: NodeIdentifier,
}

pub struct Graph<'a, T, V> {
    nodes: Vec<T>,
    edges: Vec<Vec<Edge<V>>>,
}

impl<'a, T, V> Graph<'a, T, V> {
    pub fn new() -> Graph<'a, T, V> {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn get(&self, index: NodeIdentifier) -> &T {
        &self.nodes[index]
    }

    pub fn insert(&mut self, value: T) -> NodeIdentifier {
        let node_index = self.nodes.len();
        // Add the new node
        self.nodes.push(value);
        // And its list in the edge list
        self.edges.push(Vec::new());
        node_index
    }

    pub fn connect(&mut self, from_index: uint, to_index: uint, data: V) {
        let edge_list = self.edges.get_mut(from_index);
        edge_list.push(Edge { data: data, dest: to_index });
    }

    pub fn connected(&self, from_index: uint, to_index: uint) -> bool {
        match self.connection(from_index, to_index) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn connection(&self, from_index: NodeIdentifier, to_index: NodeIdentifier) -> Option<&Edge<V>> {
        for edge in self.connections(from_index).iter() {
            if edge.dest == to_index {
                return Some(edge);
            }
        }
        None
    }

    pub fn connections(&self, for_node: NodeIdentifier) -> &Vec<Edge<V>> {
        &self.edges[for_node]
    }

    pub fn visit_breadth_first(&self, from_index: uint, visitor: |&T| -> bool) {
        // Track nodes to be visited
        let mut queue = Vec::new();
        // Track nodes we've visited
        let mut visited = HashSet::new();
        // Allow `visitor` to return false should the caller want to exit early
        // (like if this was being used for a breadth first search)
        let mut should_continue = true;
        queue.push(from_index);
        visited.insert(from_index);
        while queue.len() > 0 && should_continue {
            let current = queue.remove(0).unwrap();
            should_continue = visitor(self.get(current));
            for edge in self.connections(current).iter() {
                if !visited.contains(&edge.dest) {
                    queue.push(edge.dest);
                    visited.insert(edge.dest);
                }
            }
        }
    }
}

impl<'a, T, V> Graph<'a, T, V> where T: Eq {
    pub fn bfs(&self, from_node: uint, wanted: T) -> bool {
        let mut in_graph = false;
        self.visit_breadth_first(from_node, |value| -> bool {
            if *value == wanted {
                in_graph = true;
                false
            } else {
                true
            }
        });
        in_graph
    }
}

#[cfg(test)]
mod test {
    use super::Graph;
    use std::collections::HashSet;

    #[test]
    fn test_insertion() {
        let mut g: Graph<int, ()> = Graph::new();
        let n = g.insert(5i);
        assert!(*g.get(n) == 5i);
    }

    #[test]
    fn test_connect_nodes() {
        let mut g = Graph::new();
        let n1 = g.insert(5i);
        let n2 = g.insert(2i);
        assert!(!g.connected(n1, n2));
        g.connect(n1, n2, ());
        assert!(g.connected(n1, n2));
    }

    #[test]
    fn test_connection_data() {
        let mut g = Graph::new();
        let n1 = g.insert(1i);
        let n2 = g.insert(2i);
        g.connect(n1, n2, 1i);

        let edge = g.connection(n1, n2);
        assert!(match edge { Some(_) => true, None => false });
        assert!(edge.unwrap().data == 1i);

        let bad_edge = g.connection(n2, n1);
        assert!(match bad_edge { Some(_) => false, None => true });
    }

    #[test]
    fn test_bfs_unconnected() {
        let mut g: Graph<int, ()> = Graph::new();
        let n1 = g.insert(5i);
        g.insert(2i);
        let mut visited = Vec::new();
        g.visit_breadth_first(n1, |i| -> bool {
            visited.push(*i);
            true
        });
        assert!(visited.len() == 1);
        assert!(visited[0] == 5i);
    }

    #[test]
    fn test_bfs_connected() {
        let mut g: Graph<int, ()> = Graph::new();
        let n1 = g.insert(1i);
        let n2 = g.insert(2i);
        let n3 = g.insert(3i);
        let n4 = g.insert(4i);
        g.connect(n1, n2, ());
        g.connect(n2, n3, ());
        g.connect(n1, n4, ());

        let mut visited = Vec::new();
        g.visit_breadth_first(n1, |i| -> bool {
            visited.push(*i);
            true
        });
        // We should go along sibling nodes before decending
        assert!(visited.len() == 4);
        assert!(visited[0] == 1i);
        assert!(visited[1] == 2i);
        assert!(visited[2] == 4i);
        assert!(visited[3] == 3i);
    }

    #[test]
    fn test_bfs_loop_protect() {
        let mut g: Graph<int, ()> = Graph::new();
        let n1 = g.insert(1i);
        let n2 = g.insert(2i);
        let n3 = g.insert(3i);
        g.connect(n1, n2, ());
        g.connect(n2, n3, ());
        g.connect(n3, n1, ());

        let mut visited = HashSet::new();
        let mut already_seen = false;
        g.visit_breadth_first(n1, |i| -> bool {
            already_seen = visited.contains(i);
            visited.insert(*i);
            !already_seen
        });
        assert!(!already_seen)
    }

    #[test]
    fn test_bfs_early_exit() {
        let mut g: Graph<int, ()> = Graph::new();
        let n1 = g.insert(1i);
        let n2 = g.insert(2i);
        let n3 = g.insert(3i);
        let n4 = g.insert(4i);
        g.connect(n1, n2, ());
        g.connect(n2, n3, ());
        g.connect(n1, n4, ());

        let mut visited = Vec::new();
        g.visit_breadth_first(n1, |i| -> bool {
            visited.push(*i);
            !(*i == 2)
        });
        assert!(visited.len() == 2);
        assert!(visited[0] == 1i);
        assert!(visited[1] == 2i);
    }

    #[test]
    fn test_bfs_search() {
        let mut g: Graph<int, ()> = Graph::new();
        let n1 = g.insert(1i);
        let n2 = g.insert(2i);
        let n3 = g.insert(3i);
        let n4 = g.insert(4i);
        g.connect(n1, n2, ());
        g.connect(n2, n3, ());
        g.connect(n1, n4, ());

        assert!(g.bfs(n1, 1i));
        assert!(!g.bfs(n1, 10i));
    }
}

