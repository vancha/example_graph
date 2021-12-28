use std::collections::{HashMap, HashSet, VecDeque};

///A graph that uses an adjacency list to store vertices
struct Graph<T> {
    ///the adjacency list consists of a hashmap with a generic index pointing to a hashset of
    ///adjacent generic values
    adjacency_list: HashMap<T, HashSet<T>>,
}
///this trait can be implemented on types to show they represent little caves
trait IsSmallCave {
    fn is_small_cave(&self) -> bool;
}
///this trait represents big caves
trait IsBigCave {
    fn is_big_cave(&self) -> bool;
}
///every generic type that can display itself automatically gets the IsSmallCave trait implemented
impl<T: std::fmt::Display> IsSmallCave for T {
    fn is_small_cave(&self) -> bool {
        //turn type to string, loop over it, and if it has any uppercase letters it's *not* a
        //little cave
        self.to_string()
            .chars()
            .filter(|c| c.is_ascii_uppercase())
            .count()
            == 0
    }
}
impl<T: IsSmallCave> IsBigCave for T {
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
        //checks if this from_vertex has already been added to the graph
        if self.adjacency_list.contains_key(&from_vertex) {
            //if it has, retrieve it's value in the adjacency list mutably, and update it to add
            //the new to_vertex.
            self.adjacency_list
                .get_mut(&from_vertex)
                .unwrap()
                .insert(to_vertex);
        } else {
            //if it has not, create a new value in the adjacency list, and set it's value to be a
            //hashset containing just the new to_vertex.
            self.adjacency_list
                .insert(from_vertex, HashSet::from([to_vertex]));
        }
    }

    ///add undirected edge, goes two ways
    fn add_undirected_edge(&mut self, from_vertex: T, to_vertex: T) {
        //same as for a directed edge, add vertex to the graph, or update it if it's already added
        if self.adjacency_list.contains_key(&from_vertex) {
            self.adjacency_list
                .get_mut(&from_vertex)
                .unwrap()
                .insert(to_vertex);
        } else {
            self.adjacency_list
                .insert(from_vertex, HashSet::from([to_vertex]));
        }
        //also repeat the same the other way around, instead of connecting the from_vertex to the
        //two vertex, connect the to_vertex to the from_vertex also, a two way connection.
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
            //wrap the result in a sum, so that errors can be handled safely
            Some(
                //from the adjacency list
                self.adjacency_list
                    //retrieve the neighbours that correspond to this vertex
                    .get(&vertex)
                    .unwrap()
                    //turn the list into an iterator
                    .into_iter()
                    //make sure that instead of returning references, we return dereferenced
                    //references, i.e. just regular T elements :)
                    .map(|v| *v)
                    .collect(),
            )
        } else {
            //if the vertex does not exist, return none
            None
        }
    }
    ///print vertices in graph in depth first order
    fn print_depth_first(&self, starting_vertex: T) {
        //in dfs, add the starting vertex to a stack
        let mut stack: Vec<T> = Vec::from([starting_vertex]);
        //keep following these steps until the stack is empty
        while !stack.is_empty() {
            //take the current vertex from the top of the stack
            let current = stack.pop().unwrap();
            println!("{}", current);
            //get all neighbours for this current vertex
            if let Some(neighbours) = self.get_neighbours(current) {
                for neighbour in neighbours {
                    //push all of it's neighbours to the stack, easy :)
                    stack.push(neighbour);
                }
            }
        }
    }
    ///print vertices in graph in breadth first order
    fn print_breadth_first(&self, starting_vertex: T) {
        //for breadth first searching (or printing in this case), to the exact same, but with a
        //queue!
        //first create the queue
        let mut queue: VecDeque<T> = VecDeque::from([starting_vertex]);
        //perform these steps until the queue is empty
        while !queue.is_empty() {
            //take the last vertex off of the queue
            let current = queue.pop_front().unwrap();
            println!("{}", current);
            //get all of it's neighbours
            if let Some(neighbours) = self.get_neighbours(current) {
                for neighbour in neighbours {
                    //add them to the queue, just as easy ^^
                    queue.push_back(neighbour);
                }
            }
        }
    }

    ///count all paths between two vertices in the graph
    fn count_paths_between(
        &self,
        current_vertex: T,
        destination_vertex: T,
        visited: &mut HashMap<T, bool>,
        path: &mut Vec<T>,
        path_count: &mut i32,
    ) {
        //mark the vertex that's currently being processed as visited 
        visited.insert(current_vertex, true);
        //add this vertex to the path
        path.push(current_vertex);
        //if this vertex is in fact the destination vertex, we have finished a path
        if current_vertex == destination_vertex {
            //increment the path counter
            *path_count += 1;
            //print the path
            println!("current path: {:?}", path);
        } else {
            //in case it's not our destination vertex, we get all of the current vertex' neighbours
            for i in self.get_neighbours(current_vertex).unwrap_or(vec![]) {
                if !visited.contains_key(&i) || i.is_big_cave() {
                    self.count_paths_between(i, destination_vertex, visited, path, path_count);
                }
            }
        }
        path.pop();
        visited.remove_entry(&current_vertex);
    }

    ///count all paths between two vertices in the graph
    fn count_paths_between_caves(
        &self,
        u: T,
        destination: T,
        visited: &mut HashMap<T, bool>,
        path: &mut Vec<T>,
        nr: &mut i32,
    ) -> Vec<T> {
        visited.insert(u, true);
        path.push(u);
        if u == destination {
            *nr += 1;
            println!("current path: {:?}", path);
        } else {
            for neighbour in self.get_neighbours(u).unwrap_or(vec![]) {
                if !visited.contains_key(&neighbour) || neighbour.is_big_cave() {
                    self.count_paths_between_caves(neighbour, destination, visited, path, nr);
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
    g.add_undirected_edge("ax", "end");
    g.add_undirected_edge("xq", "GF");
    g.add_undirected_edge("end", "xq");
    g.add_undirected_edge("im", "wg");
    g.add_undirected_edge("ax", "ie");
    g.add_undirected_edge("start", "ws");
    g.add_undirected_edge("ie", "ws");
    g.add_undirected_edge("CV", "start");
    g.add_undirected_edge("ng", "wg");
    g.add_undirected_edge("ng", "ie");
    g.add_undirected_edge("GF", "ng");
    g.add_undirected_edge("ng", "av");
    g.add_undirected_edge("CV", "end");
    g.add_undirected_edge("ie", "GF");
    g.add_undirected_edge("CV", "ie");
    g.add_undirected_edge("im", "xq");
    g.add_undirected_edge("start", "GF");
    g.add_undirected_edge("GF", "ws");
    g.add_undirected_edge("wg", "LY");
    g.add_undirected_edge("CV", "ws");
    g.add_undirected_edge("im", "CV");
    g.add_undirected_edge("CV", "wg");

    //g.print_depth_first("start");//don't use on undirected thingies
    //g.print_breadth_first("start");//don't use on undirected thingies
    let mut nr = 0;
    g.count_paths_between_caves("start", "end", &mut HashMap::new(), &mut vec![], &mut nr);
    println!("{} paths:", nr);
}
