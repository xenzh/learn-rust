fn main() {
    // associating types with the trait.
    // helps to avoid everything related with original trait
    // to be generic over associated types.
    trait Graph {
    	type Node;
    	type Edge;

    	fn has_edge(&self, &Self::Node, &Self::Node) -> bool;
    	fn edges(&self, &Self::Node) ->Vec<Self::Edge>;
    }

    #[allow(dead_code)]
    fn distance<G: Graph>( _graph: &G, _start: &G::Node, _end: &G::Node ) -> u32 {
    	unimplemented!();
    }

    // implementing associated types:
    struct Node;
    struct Edge;

    #[allow(dead_code)]
    struct MyGraph;

    impl Graph for MyGraph {
    	type Node = Node;
    	type Edge = Edge;

    	fn has_edge( &self, _n1: &Node, _n2: &Node ) -> bool { true }
    	fn edges(&self, _n: &Node) -> Vec<Edge> { Vec::new() }
    }

    // providing actual assoc types for trait objects:
    let graph = MyGraph;
    let _obj = Box::new(graph) as Box<Graph<Node=Node, Edge=Edge>>;
}
