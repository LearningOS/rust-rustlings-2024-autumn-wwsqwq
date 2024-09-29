/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/
use std::collections::VecDeque;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest); 
        self.adj[dest].push(src); 
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
		//TODO
        let n = self.adj.len();  
        let mut visited = vec![false; n]; // 用于跟踪已访问的节点  
        let mut queue = VecDeque::new();  
        let mut visit_order = Vec::new();  
  
        visited[start] = true; // 标记起始节点为已访问  
        queue.push_back(start); // 将起始节点加入队列  
  
        while !queue.is_empty() {  
            let current = queue.pop_front().unwrap();  
            visit_order.push(current); // 将当前节点添加到访问顺序中  
  
            for &neighbor in &self.adj[current] {  
                if !visited[neighbor] { // 如果邻居节点未被访问  
                    visited[neighbor] = true; // 标记为已访问  
                    queue.push_back(neighbor); // 将邻居节点加入队列  
                }  
            }  
        }  
  
        visit_order  
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}

