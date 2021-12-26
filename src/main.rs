use std::collections::{HashMap, HashSet, VecDeque};

struct Graph<T> {
    adjacency_list: HashMap<T, HashSet<T>>,
}
impl<T> Graph<T>
where
    T: std::hash::Hash + Eq + std::fmt::Display + Copy,
{
    fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }
    fn add_edge(&mut self, from: T, to: T) {
        if self.adjacency_list.contains_key(&from) {
            self.adjacency_list.get_mut(&from).unwrap().insert(to);
        } else {
            self.adjacency_list.insert(from, HashSet::from([to]));
        }
    }

    fn get_neighbours(&self, element: T) -> Option<Vec<T>> {
        if self.adjacency_list.contains_key(&element) {
            Some(
                self.adjacency_list
                    .get(&element)
                    .unwrap()
                    .into_iter()
                    .map(|v| *v)
                    .collect(),
            ) //not nice, needs work
        } else {
            None
        }
    }
    fn print_depth_first(&self, starting_node: T) {
        let mut stack: Vec<T> = Vec::from([starting_node]);
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
    fn print_breadth_first(&self, starting_node: T) {
        let mut queue: VecDeque<T> = VecDeque::from([starting_node]);
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
