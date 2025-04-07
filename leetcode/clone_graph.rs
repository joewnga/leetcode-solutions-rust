/// 
/// Problem: Clone Graph
/// 
/// Given a reference of a node in a connected undirected graph, return a deep copy (clone) of the graph.
/// 
/// Each node in the graph contains a value (int) and a list (List[Node]) of its neighbors.
/// 
/// class Node {
///     public int val;
///     public List<Node> neighbors;
/// }
/// 
/// Test case format:
/// 
/// For simplicity, each node's value is the same as the node's index (1-indexed). For example, the first
/// node with val == 1, the second node with val == 2, and so on. The graph is represented in the test 
/// case using an adjacency list.
/// 
/// An adjacency list is a collection of unordered lists used to represent a finite graph. Each list 
/// describes the set of neighbors of a node in the graph.
/// 
/// The given node will always be the first node with val = 1. You must return the copy of the given 
/// node as a reference to the cloned graph.
/// 
/// Example 1:
/// Input: adjList = [[2,4],[1,3],[2,4],[1,3]]
/// Output: [[2,4],[1,3],[2,4],[1,3]]
/// Explanation: There are 4 nodes in the graph.
/// 1st node (val = 1)'s neighbors are 2nd node (val = 2) and 4th node (val = 4).
/// 2nd node (val = 2)'s neighbors are 1st node (val = 1) and 3rd node (val = 3).
/// 3rd node (val = 3)'s neighbors are 2nd node (val = 2) and 4th node (val = 4).
/// 4th node (val = 4)'s neighbors are 1st node (val = 1) and 3rd node (val = 3).
/// 
/// Example 2:
/// Input: adjList = [[]]
/// Output: [[]]
/// Explanation: Note that the input contains one node with val = 1 and an empty list of neighbors.
/// 
/// Example 3:
/// Input: adjList = []
/// Output: []
/// Explanation: This is an empty graph, it does not have any nodes.
/// 
/// Constraints:
/// The number of nodes in the graph is in the range [0, 100].
/// 1 <= Node.val <= 100
/// Node.val is unique for each node.
/// There are no repeated edges and no self-loops in the graph.
/// The Graph is connected and all nodes can be visited starting from the given node.
/// 

// Definition for a Node (as typically defined in this problem)
// #[derive(Debug, Clone)]
// pub struct Node {
//   pub val: i32,
//   pub neighbors: Vec<Option<Rc<RefCell<Node>>>>,
// }
// 
// impl Node {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     Node {
//       val,
//       neighbors: vec![],
//     }
//   }
// }

// # Solution 
// Time complexity: O(V + E) where V is the number of vertices and E is the number of edges
// Space complexity: O(V) 

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

impl Solution {
   pub fn clone_graph(node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
       if node.is_none() {
           return None;
       }
       
       // HashMap to keep track of cloned nodes: original node -> cloned node
       let mut visited: HashMap<i32, Rc<RefCell<Node>>> = HashMap::new();
       
       Self::dfs_clone(node, &mut visited)
   }
   
   fn dfs_clone(
       node: Option<Rc<RefCell<Node>>>, 
       visited: &mut HashMap<i32, Rc<RefCell<Node>>>
   ) -> Option<Rc<RefCell<Node>>> {
       if let Some(n) = node {
           let val = n.borrow().val;
           
           // If this node has already been cloned, return the clone
           if visited.contains_key(&val) {
               return Some(Rc::clone(&visited[&val]));
           }
           
           // Create a new clone with the same value but empty neighbors
           let clone = Rc::new(RefCell::new(Node::new(val)));
           
           // Mark as visited before processing neighbors to handle cycles
           visited.insert(val, Rc::clone(&clone));
           
           // Clone all neighbors
           let neighbors = &n.borrow().neighbors;
           for neighbor in neighbors {
               let cloned_neighbor = Self::dfs_clone(neighbor.clone(), visited);
               clone.borrow_mut().neighbors.push(cloned_neighbor);
           }
           
           Some(clone)
       } else {
           None
       }
   }
}