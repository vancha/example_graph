use std::collections::{HashMap, HashSet, VecDeque};

///A graph that uses an adjacency list to store vertices
struct Graph<T> {
    ///the adjacency list consists of a hashmap with a generic index pointing to a hashset of
    ///adjacent generic values
    adjacency_list: HashMap<T, HashSet<T>>,
}
///the generic objects used by the graph must implement hash, display, eq and copy
impl<T> Graph<T>
where
    T: std::hash::Hash + Eq + std::fmt::Display + Copy,
{
    ///builder pattern: easily instantiate a new graph
    fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }
    ///add directed edge, goes one way
    fn add_edge(&mut self, from_vertex: T, to_vertex: T) {
        if self.adjacency_list.contains_key(&from_vertex) {
            self.adjacency_list.get_mut(&from_vertex).unwrap().insert(to_vertex);
        } else {
            self.adjacency_list.insert(from_vertex, HashSet::from([to_vertex]));
        }
    }
    ///get neighbours for any vertex
    fn get_neighbours(&self, vertex: T) -> Option<Vec<T>> {
        if self.adjacency_list.contains_key(&vertex) {
            Some(
                self.adjacency_list
                    .get(&vertex)
                    .unwrap()
                    .into_iter()
                    .map(|v| *v)
                    .collect(),
            ) //not nice, needs work
        } else {
            None
        }
    }
    ///print vertices in graph in depth first order
    fn print_depth_first(&self, starting_vertex: T) {
        let mut stack: Vec<T> = Vec::from([starting_vertex]);
        while !stack.is_empty() {
            let current = stack.pop().unwrap();
            println!("{}", current);
            if let Some(neighbours) = self.get_neighbours(current) {
                for neighbour in neighbours {
                    stack.push(neighbour);
                }
            }
        }
    }
    ///print vertices in graph in breadth first order
    fn print_breadth_first(&self, starting_vertex: T) {
        let mut queue: VecDeque<T> = VecDeque::from([starting_vertex]);
        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            println!("{}", current);
            if let Some(neighbours) = self.get_neighbours(current) {
                for neighbour in neighbours {
                    queue.push_back(neighbour);
                }
            }
        }
    }
    ///count all paths between two vertices in the graph
    fn count_paths_between(&self, starting_node: T, ending_node: T, current: T, visited: Vec<T>)-> i32 {
        let mut stack: Vec<T> = Vec::from([starting_node]);
        while !stack.is_empty() {
            let current = stack.pop().unwrap();
            println!("{}", current);
            if let Some(neighbours) = self.get_neighbours(current) {
                for neighbour in self.get_neighbours(current).unwrap() {
                    stack.push(neighbour);
                }
            }
        }
    6
    }
}

fn main() {
    let mut g:Graph<&str> = Graph::new();
    g.add_edge("start", "A");
    g.add_edge("start", "b");
    g.add_edge("A", "c");
    g.add_edge("A", "b");
    g.add_edge("b", "d");
    g.add_edge("b", "d");
    g.add_edge("A", "end");
    g.add_edge("b", "end");
    g.print_depth_first("start");
    println!("\n");
    g.print_breadth_first("start");
    println!("nr of paths: {}",g.count_paths_between("start","end","start",vec![]))
}
