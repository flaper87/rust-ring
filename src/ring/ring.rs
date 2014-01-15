extern mod rust_crypto = "github.com/DaGenix/rust-crypto#rust-crypto:0.1";

use std::hash::Hash;
use std::hashmap::HashMap;
use std::rand::{task_rng, Rng};

use rust_crypto::md5::Md5;
use rust_crypto::digest::Digest;

struct RingPartitioned<'a> {
    name: ~str,
    nodes: HashMap<uint, HashMap<~str, uint>>,

    /// Partitioning
    shift: uint,
    replicas: uint,
    part_power: uint,
    partitions: ~[uint],
}

trait Ring {

    /**
     * Generate an id for str
     **/
    fn get_id<'a>(&'a self, id: &str) -> u32;

    /**
     * Rebalance the ring.
     *
     * Rebalances the ring after adding or removing
     * nodes to it.
     **/
    fn rebalance<'a>(&'a mut self) -> ();

    /**
     * Gets a list of nodes for `id`
     **/
    fn get_nodes<'a>(&'a self, id: &str) -> ~[&'a HashMap<~str, uint>];

    /**
     * Gets a list of nodes for `id`
     **/
    fn add_node<'a>(&'a mut self, node: HashMap<~str, uint>) -> ();
}

impl<'a> Ring for RingPartitioned<'a> {

    fn get_id<'a>(&'a self, id: &str) -> u32 {
       let mut sh = Md5::new();
       sh.input_str(id);
       sh.result_str().hash() as u32 >> self.shift
    }

    fn rebalance<'a>(&'a mut self) -> () {

        // No nodes nor partitions. Nothing
        // to rebalance.
        if self.nodes.len() == 0 && self.partitions.len() == 0 { return; }

        let node_weight = 2;
        let total_weight = 2 * self.nodes.len();

        // http://github.com/mozilla/rust/issues/11499
        // This is equivalent to 2^self.part_power
        let mut total_partitions = 1 << self.part_power;
        total_partitions -= self.partitions.len();

        let desired_parts = total_partitions / total_weight * node_weight;
        for node in self.nodes.iter() {
            if total_partitions > 0 {
               for _ in range(1, desired_parts) {
                   if total_partitions <= 0 {
                       break;
                   }
                   self.partitions.push(*node.n0());
                   total_partitions -= 1;
               }
            }
        }

        task_rng().shuffle_mut(self.partitions);
    }


    fn add_node<'a>(&'a mut self, node: HashMap<~str, uint>) -> () {
        let new_id = self.nodes.len() + 2;
        self.nodes.insert(new_id, node);
    }

    fn get_nodes<'a>(&'a self, id: &str) -> ~[&'a HashMap<~str, uint>] {
        let mut part = self.get_id(id);
        let mut nodes = ~[];
        let mut node_id = self.partitions[part];

        nodes.push(self.nodes.get(&node_id));


        while nodes.len() < self.replicas as uint {
            part += 1;
            if part as uint > self.partitions.len() {
                part = 0;
            }

            node_id = self.partitions[part];
            if nodes.contains(&self.nodes.get(&node_id)) {
                continue;
            }

            nodes.push(self.nodes.get(&node_id));
        }

        nodes
    }

}

impl<'a> RingPartitioned<'a> {

    fn new(name: ~str, part_power: uint, replicas: uint) -> RingPartitioned<'a> {

        RingPartitioned {
            name: name,
            nodes: HashMap::new(),
            replicas: replicas,
            shift: 32 - part_power,
            part_power: part_power,
            partitions: ~[]}

    }
}


#[cfg(test)]
mod tests {
    use std::hashmap::HashMap;

    use ring::Ring;
    use ring::RingPartitioned;

    fn get_ring(nodes: uint) -> RingPartitioned {
        let mut ring = RingPartitioned::new(~"test", 16, 3);

        // Add some nodes
        for nid in range(1u, nodes) {
            let mut node = HashMap::new();
            node.insert(~"id", nid);
            ring.add_node(node);
        }

        ring

    }

    #[test]
    fn test_rebalance() {
        let mut ring = get_ring(4);
        ring.rebalance();
        assert_eq!(ring.partitions.len(), 65529)
    }

    #[test]
    fn test_get_nodes() {
        let mut ring = get_ring(4);
        ring.rebalance();
        let nodes = ring.get_nodes("0");
        assert_eq!(nodes.len(), 3)
    }
}
