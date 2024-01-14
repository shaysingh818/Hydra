use crate::ndarray::ndarray::NDArray;
use std::fs::File;
use std::io::{BufWriter, Read};


pub trait Ops {
    fn save(&self, filepath: &str) -> std::io::Result<()>;
    fn load(filepath: &str) -> std::io::Result<NDArray<f64>>;
    fn add(&self, other: NDArray<f64>) -> Result <NDArray<f64 >, String>;
    fn subtract(&self, other: NDArray<f64>) -> Result<NDArray<f64>, String>;
    fn scale_add(&self, other: NDArray<f64>) -> Result<NDArray<f64>, String>;
    fn transpose(self) -> Result<NDArray<f64>, String>;
    fn permute(self, indice_order: Vec<usize>) -> Result<NDArray<f64>, String>;  
}

impl Ops for NDArray<f64> {

    fn save(&self, filepath: &str) -> std::io::Result<()> {
        let filename_format = format!("{filepath}.json");
        let file = match File::create(filename_format) {
            Ok(file) => file,
            Err(err) => {
                return Err(err);
            }
        };
        let mut writer = BufWriter::new(file);
        let _ = serde_json::to_writer(&mut writer, &self);
        Ok(())
    }


    fn load(filepath: &str) -> std::io::Result<NDArray<f64>> {
        let filename_format = format!("{filepath}.json");
        let mut file = match File::open(filename_format) {
            Ok(file) => file,
            Err(err) => {
                return Err(err);
            }
        };
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let instance: NDArray<f64> = serde_json::from_str(&contents)?;
        Ok(instance)
    }

    /* ops */ 
    fn add(&self, value: NDArray<f64>) -> Result<NDArray<f64>, String> {

        /* rank mismatch */
        if self.rank() != value.rank() {
            return Err("Add: Rank Mismatch".to_string());
        }

        let curr_shape: &Vec<usize> = self.shape();
        let mut result = NDArray::new(curr_shape.to_vec()).unwrap();
        if self.size() != value.values().len() {
            return Err("Add: Size mismatch for arrays".to_string());
        }

        let mut counter = 0; 
        let values = value.values(); 
        for item in self.values() {
            let add_result = item + values[counter];
            let _ = result.set_idx(counter, add_result);
            counter += 1;
        }

        Ok(result)
    }

    fn subtract(&self, value: NDArray<f64>) -> Result<NDArray<f64>, String> {

        /* rank mismatch */
        if self.rank() != value.rank() {
            return Err("Subtract: Rank Mismatch".to_string());
        }

        let curr_shape: &Vec<usize> = self.shape();
        let mut result = NDArray::new(curr_shape.to_vec()).unwrap();
        if self.size() != value.values().len() {
            return Err("Subtract: Size mismatch for arrays".to_string());
        }

        let mut counter = 0; 
        let values = value.values(); 
        for item in self.values() {
            let add_result = item - values[counter];
            let _ = result.set_idx(counter, add_result);
            counter += 1;
        }

        Ok(result)
    }

    fn scale_add(&self, value: NDArray<f64>) -> Result<NDArray<f64>, String> {

        let value_shape = value.shape();
        if value_shape[0] != 1 {
            return Err("Scale add must have a vector dimension (1, N)".to_string());
        }

        let mut total_counter = 0; 
        let mut counter = 0;
        let vector_values = value.values();
        let curr_shape: &Vec<usize> = self.shape();
        let mut result = NDArray::new(curr_shape.to_vec()).unwrap();
        for item in self.values() {
            if counter == value.size() {
                counter = 0;
            }
             let add_result = item + vector_values[counter];
             let _ = result.set_idx(total_counter, add_result);
             total_counter += 1; 
        }

        Ok(result)

    }

    fn transpose(self) -> Result<NDArray<f64>, String> {

        if self.rank() != 2 {
            return Err("Transpose must contain on rank 2 values".to_string());
        }

        let mut index = 0;
        let shape = self.shape();
        let mut reversed_shape = shape.clone();  
        reversed_shape.reverse();
        let mut result = NDArray::new(reversed_shape.to_vec()).unwrap();

        for _item in self.values() {

            let indices = self.indices(index).unwrap();
            let mut reversed_indices = indices.clone();
            reversed_indices.reverse();

            let idx = self.index(indices).unwrap();
            let val = self.values()[idx]; 

            /* set value from reversed */ 
            let _ = result.set(reversed_indices ,val);
            index += 1; 
        }

        Ok(result)

    }

    fn permute(self, indice_order: Vec<usize>) -> Result<NDArray<f64>, String> {

        if indice_order.len() != self.rank() {
            return Err("Indice order must be same length as rank".to_string());
        }

        let mut index = 0; 
        let curr_shape = self.shape();
        let mut new_shape = Vec::new();
        for item in &indice_order {
            new_shape.push(curr_shape[*item]);
        }

        let mut result = NDArray::new(new_shape).unwrap();
        for _item in self.values() {

            let indices = self.indices(index).unwrap();
            let mut new_indice_order = Vec::new();
            for item in &indice_order {
                new_indice_order.push(indices[*item])
            }

            let idx = self.index(indices.clone()).unwrap();
            let val = self.values()[idx]; 

            /* set value from reversed */ 
            let _ = result.set(new_indice_order ,val);
            index += 1; 
        }

        Ok(result)
    }

}