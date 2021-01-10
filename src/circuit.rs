use std::collections::{HashMap,HashSet};
use crate::matrix::Matrix;
use crate::utils::extensions::{MultEsc,SumVecEq};

#[derive(Clone)]
struct Branch {
    component: String,
    start: i32,
    end: i32,
    value: f64
}

pub struct Circuit {
    branches: Vec<Branch>,
    supernodes: Vec<Branch>,
    grounded_voltages: Vec<Branch>,
    node_names : Vec<String>,
    voltage_nodes_gnd : HashSet<i32>,
    dimension: usize
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
        
        let mut node_names : Vec<_> = node_names.into_iter().collect();
        node_names.sort_unstable();
        let dimension = node_names.len() ;
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

    pub fn solve(self)  {
        let mut mat_a = Matrix::new(self.dimension,self.dimension);
        let mut mat_b = Matrix::new(self.dimension,1);
        let mut equations : Vec<Vec<Branch>> = vec![vec![];self.dimension];
        for branch in self.branches.iter() {
            equations[branch.start as usize -1].push(branch.clone());
            if branch.end != 0 {
                equations[branch.end as usize -1].push(branch.clone());
            }
        }
        for (eq,branches) in equations.iter().enumerate() {
            for branch in branches.iter() {
                if branch.component.contains("r") {
                    if eq as i32 == branch.start-1 {
                        if !self.voltage_nodes_gnd.contains(&branch.start){
                            mat_a[eq][branch.start as usize-1 ] += 1.0/branch.value;
                        }
                        if self.voltage_nodes_gnd.contains(&branch.start) && branch.start-1 != eq as i32 {
                            let val = self.get_voltage_gnd(branch.start) * 1.0/branch.value; // Ajustando valor por fuente de voltaje conectada a tierra
                            mat_b[eq][0] += val;
                        }
                        if !self.voltage_nodes_gnd.contains(&branch.end) && branch.end!=0 {
                            mat_a[eq][branch.end as usize-1] +=  -1.0/branch.value;
                        }
                        if self.voltage_nodes_gnd.contains(&branch.end) && branch.end-1 != eq as i32 { // Ajustando valor por fuente de voltaje conectada a tierra
                            let val = self.get_voltage_gnd(branch.end) * -1.0/branch.value;
                            mat_b[eq][0] += val;
                        }
                    } else {
                        if !self.voltage_nodes_gnd.contains(&branch.start) {
                            mat_a[eq][branch.start as usize-1] += -1.0/branch.value;
                         }   
                        if self.voltage_nodes_gnd.contains(&branch.start) && branch.start-1 != eq as i32 { 
                            let val = self.get_voltage_gnd(branch.start) * -1.0/branch.value; // Ajustando valor por fuente de voltaje conectada a tierra 
                            mat_b[eq][0] += val;
                        }
                        if !self.voltage_nodes_gnd.contains(&branch.end) && branch.end!=0 {
                            mat_a[eq][branch.end as usize-1] +=  1.0/branch.value;
                        }
                        if self.voltage_nodes_gnd.contains(&branch.end) && branch.end-1 != eq as i32 { // Ajustando valor por fuente de voltaje conectada a tierra 
                            let val = self.get_voltage_gnd(branch.end) * 1.0/branch.value;
                            mat_b[eq][0] += val;
                        }                    
                    }
                } else if branch.component.contains("i") { // es una fuente de corriente
                    if eq as i32 == branch.start-1 { // es una corriente que sale del nodo
                        mat_b[eq][0] += -branch.value;
                    }    
                    else {// es una corriente que entra al nodo
                        mat_b[eq][0] += branch.value;
                    }
                }
            }
        }
        //Se procesa las ramas que son supernodos
        for branch in self.supernodes.iter() {
            mat_a[branch.start as  usize -1] = mat_a[branch.start as  usize -1].clone().sum_vec_eq(mat_a[branch.end as  usize -1].clone());
            mat_b[branch.start as  usize -1][0] += mat_b[branch.end as  usize -1][0];
            mat_b[branch.end as  usize -1][0] =  -1.0*branch.value;
            mat_a[branch.end as  usize -1] =  mat_a[branch.end as  usize -1].clone().mult_esc(0.0);
            mat_a[branch.end as  usize -1][branch.start as  usize -1] = 1.0;
            mat_a[branch.end as  usize -1][branch.end as  usize -1] = -1.0;
        }
        // Se procesa los nodos con fuentes de voltaje conectadas a tierra
        for branch in self.grounded_voltages.iter() {
            mat_a[branch.start as usize-1] = mat_a[branch.start as  usize -1].clone().mult_esc(0.0);
            mat_a[branch.start as usize-1][branch.start as usize-1] = 1.0 ;
            mat_b[branch.start as usize-1][0] = branch.value;
        }
        mat_a.print_mat("A:");
        mat_b.print_mat("b:");
        let inv = mat_a.inverse();
        let res = inv.multiply(&mat_b);
        println!("Result: [V]");
        for (node,res) in self.node_names.iter().zip(res.data.iter()) {
            println!("{}: {1:.4}",node,res[0]);
        }
    }
}