use std::ops::{Index,IndexMut};
use crate::utils::extensions::{IsZero,MultEsc,SumVecEq};

pub struct Matrix {
    row: usize,
    col: usize,
    is_mat_ok: bool,
    pub data: Vec<Vec<f64>>
}

impl Matrix {
    pub fn new(row: usize, col: usize) -> Self {
        let mut data = Vec::new();
        for _r in 0..row  {
            let mut temp_row = Vec::new();
            for _c in 0..col {
                let val = 0.0;
                temp_row.push(val);
            }
            data.push(temp_row);
        }
        Self {
            row,
            col,
            is_mat_ok: true,
            data
        }
    }
    
    pub fn print_mat(&self, name: &str){
        println!("-------- Matrix {} --------\n", name);
        for r in &self.data {
            for val in r  {
                let val = *val;
                print!("{0: <10.4} | ", val );
                
            }
            println!();
        }
        println!("---------------------------");
    }

    pub fn transpose(&self) -> Self {
        let mut matc = Matrix::new(self.col, self.row);
        for r in 0..matc.row  {
            for c in 0..matc.col  {
                matc[r][c] = self[c][r];
            }
        }
        matc
    }

    pub fn multiply(&self, other: &Self) -> Self {
        let mut matc = Matrix::new(self.row, other.col);
        let transp = other.transpose();
        for r in 0..matc.row  {
            for c in 0..matc.col  {
                for y in 0..self[r].len()  {
                    matc[r][c] += self[r][y] * 
                        transp[c][y];
                }
            }
        }
        matc
    }

    pub fn inverse(&self) -> Self {
        let mut mat_ai = Matrix::new(self.row,self.col*2);
        let mut inv = Matrix::new(self.row,self.col);
        for i in 0..mat_ai.row {
            for c in 0..mat_ai.col {
                if c<self.col{ 
                    mat_ai[i][c] = self[i][c];
                }
                else if c==(self.col+i){
                    mat_ai[i][c] = 1.0;
                }
            }
        }
        for  r in 0..self.row {
            //pivot
            // is_zero, mult_esc, sum_vec_eq are Extension methods (trait implemented)
            if mat_ai[r][r].is_zero() {    
                let mut state = false;
                for i in (r+1)..self.row{
                    if !mat_ai[i][r].is_zero() {
                        let temp = mat_ai[r].clone();
                        mat_ai[r] = mat_ai[i].clone();
                        mat_ai[i] = temp.clone();
                        state = true;
                        break;
                    }
                }
                inv.is_mat_ok = state;
            }
            mat_ai[r] = mat_ai[r].clone().mult_esc(1.0/mat_ai[r][r]);
            for  c in 0..self.row {
                if r!=c {
                    mat_ai[c] = mat_ai[r].clone().mult_esc(-1.0*mat_ai[c][r])
                        .sum_vec_eq(mat_ai[c].clone());
                }
            }
        }
        for r in 0..self.row{
            for c in self.row..self.row*2{
                inv[r][(c-self.row)] = mat_ai[r][c];
            }
        }
        inv
    }

}

impl Index<usize> for Matrix {
    type Output = Vec<f64>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
