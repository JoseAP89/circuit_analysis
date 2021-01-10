use std::collections::{HashMap,HashSet};
extern crate ndarray;
use ndarray::arr2;

#[derive(Clone)]
struct Branch {
    component: String,
    start: i32,
    end: i32,
    value: f64
}

struct Circuit {
    branches: Vec<Branch>,
    supernodes: Vec<Branch>,
    grounded_voltages: Vec<Branch>,
    node_names : Vec<String>,
    voltage_nodes_gnd : HashSet<i32>,
    dimension: i32
}

impl Circuit {
    pub fn new(data: Vec<String>) -> Self {
        let mut branches = vec![];
        let mut supernodes = vec![];
        let mut grounded_voltages = vec![];
        for b in data.into_iter() {
            let tmp : Vec<String> = b.split_whitespace().
                map(|x| x.to_owned()).collect();
            let component = tmp[0].to_lowercase().clone();
            let start = tmp[1].parse::<i32>().unwrap();
            let end = tmp[2].parse::<i32>().unwrap();
            let value = tmp[3].parse::<f64>().unwrap();
            let branch = Branch {
                component,
                start,
                end,
                value
            };
            if branch.component.contains("v") {
                if branch.start!=0 && branch.end!= 0 {
                    supernodes.push(branch.clone());
                } else {
                    grounded_voltages.push(branch.clone());
                }
            }
            branches.push(branch);
            
        }
        let mut node_names = HashSet::new();
        let mut voltage_nodes_gnd = HashSet::new();
        for branch in branches.iter() {
            let node = "V".to_owned() + &branch.start.to_string();
            node_names.insert(node);
        }
        for branch in grounded_voltages.iter() {
            let node = branch.start;
            voltage_nodes_gnd.insert(node);
        }
        let dimension = voltage_nodes_gnd.len() as i32;
        let mut node_names : Vec<_> = node_names.into_iter().collect();
        node_names.sort_unstable();
        Circuit {
            branches,
            supernodes,
            grounded_voltages,
            node_names,
            voltage_nodes_gnd,
            dimension
        }
    }

    fn get_voltage_gnd(&self,node: i32) -> f64 {
        for branch in self.grounded_voltages.iter() {
            if branch.start == node {
                return branch.value;
            }
        }
        0.0
    }


}