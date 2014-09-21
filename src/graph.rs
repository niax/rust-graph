use std::collections::HashSet;
use std::collections::PriorityQueue;
use std::fmt;
use std::num::zero;
use std::slice::Items;

/// Identifier for nodes in a Graph
pub type NodeIdentifier = uint;

/// Represents a directed edge within a Graph
pub struct Edge<V> {
    pub data: V,
    pub dest: NodeIdentifier,
}

/// Base Graph implementation
pub struct Graph<'a, T, V> {
    nodes: Vec<T>,
    edges: Vec<Vec<Edge<V>>>,
}

impl<'a, T, V> Graph<'a, T, V> {
    /// Create a new Graph
    ///
    /// * `T` is the data type to be stored on the Nodes
    /// * `V` is the data type to be stored on the Edges
    pub fn new() -> Graph<'a, T, V> {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    /// Get the value for the given NodeIdentifier
    pub fn get(&self, index: NodeIdentifier) -> &T {
        &self.nodes[index]
    }

    /// Insert a value into the graph, returns the node identifier
    ///
    /// # Example
    ///
    /// ```rust
    /// use graph::Graph;
    ///
    /// let mut g: Graph<int, ()> = Graph::new();
    /// let n1 = g.insert(5i);
    /// assert!(*g.get(n1) == 5i);
    /// ```
    pub fn insert(&mut self, value: T) -> NodeIdentifier {
        let node_index = self.nodes.len();
        // Add the new node
        self.nodes.push(value);
        // And its list in the edge list
        self.edges.push(Vec::new());
        node_index
    }

    /// Connect two nodes in the graph.
    ///
    /// This is a directed edge, with the edge going from `from_index` going to `to_index`.
    /// The `data` is attached to the edge.
    pub fn connect(&mut self, from_index: NodeIdentifier, to_index: NodeIdentifier, data: V) {
        let edge_list = self.edges.get_mut(from_index);
        edge_list.push(Edge { data: data, dest: to_index });
    }

    /// Test if two nodes are connected
    ///
    /// True if the node represented by `from_index` has an edge from it to the node identified
    /// by `to_index`.
    pub fn connected(&self, from_index: NodeIdentifier, to_index: NodeIdentifier) -> bool {
        self.connection(from_index, to_index).is_some()
    }

    /// Get the edge between nodes.
    ///
    /// `None` is used to imply that there is no edge between these nodes.
    pub fn connection(&self, from_index: NodeIdentifier, to_index: NodeIdentifier) -> Option<&Edge<V>> {
        for edge in self.connections(from_index).iter() {
            if edge.dest == to_index {
                return Some(edge);
            }
        }
        None
    }

    /// Get a complete list of edges from the node indentified by `for_node`
    pub fn connections(&self, for_node: NodeIdentifier) -> &Vec<Edge<V>> {
        &self.edges[for_node]
    }

    /// Traverse the graph, visiting each node directly, or indirectly, connected to the
    /// node identified by `from_index`.
    /// 
    /// Calls `visitor` with the value of each node on the trip,
    /// visiting sibling nodes first before decending deeper into the graph.
    /// In the event of cycles, each node is visited only once.
    pub fn visit_breadth_first(&self, from_index: NodeIdentifier, visitor: |NodeIdentifier, &T| -> bool) {
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
            should_continue = visitor(current, self.get(current));
            for edge in self.connections(current).iter() {
                if !visited.contains(&edge.dest) {
                    queue.push(edge.dest);
                    visited.insert(edge.dest);
                }
            }
        }
    }

    /// Iterate over the nodes of the graph
    pub fn iter(&'a self) -> Items<'a, T> {
        self.nodes.iter()
    }

    /// Get the number of nodes in the graph
    pub fn node_count(&self) -> uint {
        self.nodes.len()
    }
}


impl<'a, T: Eq, V> Graph<'a, T, V> {
    /// Search the graph for a node with the value `wanted`.
    /// Returns true if the value is in the graph, false otherwise.
    ///
    /// Starting at `from_node`, traverse the graph, breadth first looking for the wanted
    /// value.
    pub fn bfs(&self, from_node: NodeIdentifier, wanted: T) -> Option<NodeIdentifier> {
        let mut result = None;
        self.visit_breadth_first(from_node, |node_id, value| -> bool {
            if *value == wanted {
                result = Some(node_id);
                false
            } else {
                true
            }
        });
        result
    }

    /// Searches the graph for the first node with the `value` given.
    /// 
    /// Note that this is different from `bfs` which conserns itself
    /// with connectedness.
    pub fn contains(&self, value: T) -> Option<NodeIdentifier> {
        self.contains_ref(&value)
    }

    pub fn contains_ref(&self, value: &T) -> Option<NodeIdentifier> {
        self.nodes.iter().position(|v| { v == value })
    }
}

impl<'a, T: Clone, V> Graph<'a, T, V> {
    /// Insert a slice of values into the graph
    ///
    /// Inserts clones of each of the values in the slice given into the graph
    /// Returns a vector of identifiers for the nodes inserted, in the order
    /// of the values that were input.
    pub fn insert_all(&mut self, values: &[T]) -> Vec<NodeIdentifier> {
        let mut node_indexes: Vec<NodeIdentifier> = Vec::new();
        for value in values.iter() {
            node_indexes.push(self.insert(value.clone()));
        }
        node_indexes
    }
}

impl<'a, T, V: Clone> Graph<'a, T, V> {
    /// Connects many nodes together
    ///
    /// Each tuple in the input makes a new connection, with the first element being
    /// the from node, the second being a target node and the third being the data stored
    /// on the created edge.
    ///
    /// Note that the data will be cloned onto the edge.
    pub fn connect_all(&mut self, connections: &[(NodeIdentifier, NodeIdentifier, V)]) {
        for conn in connections.iter() {
            self.connect(
                conn.ref0().clone(),
                conn.ref1().clone(),
                conn.ref2().clone(),
            );
        }
    }
}

impl<'a, T: fmt::Show, V: fmt::Show> fmt::Show for Graph<'a, T, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Nodes: {}\nEdges: {}", self.nodes, self.edges));
        Ok(())
    }
}

impl<V: fmt::Show> fmt::Show for Edge<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "d({}) -> {}", self.data, self.dest));
        Ok(())
    }
}

// Structure used when calculating shortest path
#[deriving(Eq, PartialEq)]
struct NodeCost<V> {
    node: NodeIdentifier,
    cost: V,
}

impl<V: Ord> Ord for NodeCost<V> {
    fn cmp(&self, other: &NodeCost<V>) -> Ordering {
        // XXX: EVIL TRICKS!
        // This means that 9 < 8 because the cmp is reversed
        other.cost.cmp(&self.cost)
    }
}

impl<V: Ord> PartialOrd for NodeCost<V> {
    fn partial_cmp(&self, other: &NodeCost<V>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Structure used as the return value of calculating the shortest path
pub struct ShortestPathResult<V> {
    /// The node identifiers for each node in the calculated path
    pub path: Vec<NodeIdentifier>,
    /// The cost of the path, calculated as the sum of edge data for
    /// each edge on the path
    pub cost: V,
}


impl<'a, T, V: Clone + Ord + PartialOrd + Eq + Unsigned> Graph<'a, T, V> {
    /// Finds the shortest path between two nodes.
    ///
    /// Uses Dijkstra's shortest path to connect two nodes with the least cost.
    /// Edge data is used as the "cost" metric, where a greater value incurs more cost.
    ///
    /// The returned `Option` has `Some` if a route can be calculated, containing a structure
    /// that has the path taken and the total cost of the path or `None` if there was no
    /// path between nodes.
    pub fn shortest_path(&self, from_node: NodeIdentifier, to_node: NodeIdentifier) -> Option<ShortestPathResult<V>> {
        // Current shortest path to node
        let mut dist = Vec::from_elem(self.node_count(), None);
        let mut prev = Vec::from_elem(self.node_count(), None);

        // Current nodes to consider
        let mut pq = PriorityQueue::new();

        *dist.get_mut(from_node) = Some(zero::<V>());
        pq.push(NodeCost { cost: zero::<V>(), node: from_node });
        while pq.len() > 0 {
            // Get the current lowest cost node on the fringe
            let current = pq.pop().unwrap();

            // If we've found our target, break out as we won't find a shorter path
            if current.node == to_node {
                break
            }

            // Otherwise, look at each edge to see if it offers a shorter path
            // to another node than we've already seen.
            for edge in self.connections(current.node).iter() {
                // Calculate the cost to the node pointed to by the edge,
                // if it were to go from the starting point through the node being considered
                let cost_to_node = current.cost + edge.data;

                // Figure out if we should update the queue and mapping of shortest paths
                // This happens if the path is shorter than the already found path or
                // if there has been no path found (as indicated by None)
                let should_update_cost = match dist[edge.dest] {
                    Some(ref cost) => &cost_to_node < cost,
                    None => true
                };

                // Update the queue and the shortest path for the node if we should
                if should_update_cost {
                    *dist.get_mut(edge.dest) = Some(cost_to_node.clone());
                    *prev.get_mut(edge.dest) = Some(current.node);
                    pq.push(NodeCost { cost: cost_to_node, node: edge.dest });
                }
            }
        }

        // Calculate path back based on shortest paths
        if dist[to_node].is_some() {
            let mut path = Vec::new();
            let mut current = to_node;

            // prev has the node that has the shortest path
            // to current.
            // Loop over it, tracing our path back.
            while current != from_node {
                path.push(current);
                let next = prev[current].unwrap();
                current = next;
            }
            path.push(from_node);
            path.reverse();
            Some(ShortestPathResult {
                cost: dist[to_node].clone().unwrap(),
                path: path
            })
        } else {
            None
        }
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
        assert!(match edge { Some(ref edge) => edge.data == 1i, None => false });

        let bad_edge = g.connection(n2, n1);
        assert!(match bad_edge { Some(_) => false, None => true });
    }

    #[test]
    fn test_bfs_unconnected() {
        let mut g: Graph<int, ()> = Graph::new();
        let n1 = g.insert(5i);
        g.insert(2i);
        let mut visited = Vec::new();
        g.visit_breadth_first(n1, |_, i| -> bool {
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
        g.visit_breadth_first(n1, |_, i| -> bool {
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
        g.visit_breadth_first(n1, |_, i| -> bool {
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
        g.visit_breadth_first(n1, |_, i| -> bool {
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
        let nodes = g.insert_all([0i, 1, 2, 3]);
        g.connect_all([
            (nodes[0], nodes[1], ()),
            (nodes[1], nodes[2], ()),
            (nodes[0], nodes[3], ())
        ]);

        assert!(g.bfs(nodes[0], 1i).is_some());
        assert!(g.bfs(nodes[0], 10i).is_none());
    }

    #[test]
    fn test_shortest_path() {
        let mut g = Graph::new();
        let nodes = g.insert_all([0i, 1, 2, 3]);
        g.connect_all([
            (nodes[0], nodes[1], 5u),
            (nodes[1], nodes[2], 5u),
            (nodes[0], nodes[3], 10u),
            (nodes[1], nodes[3], 3u)
        ]);

        let r1 = g.shortest_path(nodes[0], nodes[0]).unwrap();
        let r2 = g.shortest_path(nodes[0], nodes[3]).unwrap();

        assert!(r1.cost == 0u);
        assert!(r1.path == vec![0u]);
        assert!(r2.cost == 8u);
        assert!(r2.path == vec![0u, 1, 3]);
    }

    #[test]
    fn test_unconnected_shortest_path() {
        let mut g: Graph<int, uint> = Graph::new();
        let n1 = g.insert(0i);
        let n2 = g.insert(1i);

        assert!(!g.connected(n1, n2));
        assert!(g.shortest_path(n1, n2).is_none());
    }

}

