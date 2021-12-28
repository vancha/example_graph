use std::collections::{HashMap, HashSet, VecDeque};

///A graph that uses an adjacency list to store vertices
struct Graph<T> {
    ///the adjacency list consists of a hashmap with a generic index pointing to a hashset of
    ///adjacent generic values
    adjacency_list: HashMap<T, HashSet<T>>,
}
trait IsSmallCave {
    fn is_small_cave(&self) -> bool;
}
trait IsBigCave {
    fn is_big_cave(&self)->bool;
}
impl<T: std::fmt::Display> IsSmallCave for T {
    fn is_small_cave(&self) -> bool {
        self.to_string()
            .chars()
            .filter(|c| c.is_ascii_uppercase())
            .count()
            == 0
    }
}
impl <T: IsSmallCave> IsBigCave for T {
    fn is_big_cave(&self) -> bool {
        !self.is_small_cave()
    }
}

///the generic objects used by the graph must implement hash, display, eq and copy
impl<T> Graph<T>
where
    T: std::hash::Hash + Eq + std::fmt::Display + std::fmt::Debug + Copy + ToString,
{
    ///builder pattern: easily instantiate a new graph
    fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }
    ///add directed edge, goes one way
    fn add_directed_edge(&mut self, from_vertex: T, to_vertex: T) {
        if self.adjacency_list.contains_key(&from_vertex) {
            self.adjacency_list
                .get_mut(&from_vertex)
                .unwrap()
                .insert(to_vertex);
        } else {
            self.adjacency_list
                .insert(from_vertex, HashSet::from([to_vertex]));
        }
    }

    ///add undirected edge, goes two ways
    fn add_undirected_edge(&mut self, from_vertex: T, to_vertex: T) {
        if self.adjacency_list.contains_key(&from_vertex) {
            self.adjacency_list
                .get_mut(&from_vertex)
                .unwrap()
                .insert(to_vertex);
        } else {
            self.adjacency_list
                .insert(from_vertex, HashSet::from([to_vertex]));
        }
        if self.adjacency_list.contains_key(&to_vertex) {
            self.adjacency_list
                .get_mut(&to_vertex)
                .unwrap()
                .insert(from_vertex);
        } else {
            self.adjacency_list
                .insert(to_vertex, HashSet::from([from_vertex]));
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
    fn count_paths_between(
        &self,
        u: T,
        d: T,
        visited: &mut HashMap<T, bool>,
        path: &mut Vec<T>,
        path_count: &mut i32,
    ) {
        visited.insert(u, true);
        path.push(u);
        if u == d {
            *path_count += 1;
            println!("current path: {:?}", path);
        } else {
            if let Some(neighbours) = self.get_neighbours(u) {
                for i in neighbours {
                    if !visited.contains_key(&i) || i.is_big_cave() {
                        self.count_paths_between(i, d, visited, path, path_count);
                    }
                }
            }
        }
        path.pop();
        visited.remove_entry(&u);
    }

    ///count all paths between two vertices in the graph
    fn count_paths_between_caves(
        &self,
        u: T,
        d: T,
        visited: &mut HashMap<T, bool>,
        path: &mut Vec<T>,
        nr: &mut i32,
    )->Vec<T> {
        visited.insert(u, true);
        path.push(u);
        if u == d {
            *nr += 1;
            println!("current path: {:?}", path);
        } else {
            if let Some(neighbours) = self.get_neighbours(u) {
                for i in neighbours {
                    if !visited.contains_key(&i) || i.is_big_cave() {
                        self.count_paths_between_caves(i, d, visited, path,nr);
                    }
                }
            }
        }
        path.pop();
        visited.remove_entry(&u);
        path.to_vec()
    }
}

fn main() {
    let mut g: Graph<&str> = Graph::new();
    g.add_undirected_edge("ax","end");
g.add_undirected_edge("xq","GF");
g.add_undirected_edge("end","xq");
g.add_undirected_edge("im","wg");
g.add_undirected_edge("ax","ie");
g.add_undirected_edge("start","ws");
g.add_undirected_edge("ie","ws");
g.add_undirected_edge("CV","start");
g.add_undirected_edge("ng","wg");
g.add_undirected_edge("ng","ie");
g.add_undirected_edge("GF","ng");
g.add_undirected_edge("ng","av");
g.add_undirected_edge("CV","end");
g.add_undirected_edge("ie","GF");
g.add_undirected_edge("CV","ie");
g.add_undirected_edge("im","xq");
g.add_undirected_edge("start","GF");
g.add_undirected_edge("GF","ws");
g.add_undirected_edge("wg","LY");
g.add_undirected_edge("CV","ws");
g.add_undirected_edge("im","CV");
g.add_undirected_edge("CV","wg");

    //g.print_depth_first("start");//don't use on undirected thingies
    //g.print_breadth_first("start");//don't use on undirected thingies
    let mut nr = 0;
    g.count_paths_between_caves(
        "start",
        "end",
        &mut HashMap::new(),
        &mut vec![],
        &mut nr,
    );
    println!("{} paths:", nr);
}
