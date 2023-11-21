pub mod graph {

    pub mod graph_items {

        #[derive(Clone, Debug, PartialEq)]
        pub struct Attribute {
            pub key: String,
            pub value: String,
        }

        pub mod node {
            use crate::graph::graph_items::Attribute;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Node {
                pub name: String,
                pub attrs: Vec<Attribute>,
            }

            impl Node {
                pub fn new(name: &str) -> Node {
                    Node {
                        name: name.to_string(),
                        attrs: Vec::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Node {
                    self.attrs = attrs
                        .iter()
                        .map(|(k, v)| Attribute {
                            key: k.to_string(),
                            value: v.to_string(),
                        })
                        .collect();
                    self
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    for attr in &self.attrs {
                        if attr.key == key {
                            return Some(&attr.value);
                        }
                    }
                    None
                }
            }
        }

        pub mod edge {
            use crate::graph::graph_items::Attribute;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge {
                pub start: String,
                pub end: String,
                pub attrs: Vec<Attribute>,
            }
            impl Edge {
                pub fn new(start: &str, end: &str) -> Edge {
                    Edge {
                        start: start.to_string(),
                        end: end.to_string(),
                        attrs: Vec::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Edge {
                    self.attrs = attrs
                        .iter()
                        .map(|(k, v)| Attribute {
                            key: k.to_string(),
                            value: v.to_string(),
                        })
                        .collect();
                    self
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    for attr in &self.attrs {
                        if attr.key == key {
                            return Some(&attr.value);
                        }
                    }
                    None
                }
            }
        }
    }

    use crate::graph::graph_items::edge::Edge;
    use crate::graph::graph_items::node::Node;
    use std::collections::HashMap;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Graph {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Graph {
            self.nodes = nodes.clone();
            self
        }

        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Graph {
            self.edges = edges.clone();
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Graph {
            self.attrs = attrs
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();
            self
        }

        pub fn node(&self, name: &str) -> Option<&Node> {
            for node in &self.nodes {
                if node.name == name {
                    return Some(node);
                }
            }
            None
        }

        pub fn nodes(&self) -> &Vec<Node> {
            &self.nodes
        }

        pub fn edges(&self) -> &Vec<Edge> {
            &self.edges
        }

        pub fn attrs(&self) -> &HashMap<String, String> {
            &self.attrs
        }
    }
}
