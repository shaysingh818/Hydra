use hydra::structures;

#[cfg(test)]
mod ndarray {

    use crate::structures::ndarray::NDArray;

    #[test]
    fn test_create_ndarray() {

        let x: NDArray<f64> = NDArray::new(vec![3, 4]).unwrap();
        let shape = x.shape();
        let rank = x.rank();
        let expected_shape : Vec<usize> = vec![3, 4];

        /* asserts */ 
        assert_eq!(rank, 2); 
        assert_eq!(shape, &expected_shape);
    }

    #[test]
    fn test_create_ndarray_values() {

        let n: NDArray<f64> = NDArray::array(vec![2, 2], vec![0.0,0.0,1.0,1.0]).unwrap();
        let shape = n.shape();
        let rank = n.rank();
        let values = n.values();
        let expected_shape : Vec<usize> = vec![2, 2];
        let expected_size = 4; 
        let expected_vals = vec![0.0,0.0,1.0,1.0];

        /* asserts */ 
        assert_eq!(shape, &expected_shape);
        assert_eq!(rank, 2);
        assert_eq!(values, &expected_vals);
        assert_eq!(expected_size, values.len()); 

        /* value mismatch */ 
        let x1: Result<NDArray<f64>, String> = NDArray::array(vec![3, 4], vec![0.0,0.0,0.0,0.0,1.0,2.0]);
        let expected_error = "Values don't match size based on dimensions"; 
        assert_eq!(x1, Err(expected_error.to_string()));  
    }

    #[test]
    fn test_reshape() {

        /* valid reshape */ 
        let mut n: NDArray<f64> = NDArray::array(vec![2, 3], vec![0.0,0.0,1.0,1.0,2.0,2.0]).unwrap();
        let _ = n.reshape(vec![3, 2]); 
        assert_eq!(n.shape(), &vec![3, 2]);

        /* rank mismatch */
        let mut x: NDArray<f64> = NDArray::array(vec![2, 4], vec![0.0,0.0,1.0,1.0,2.0,2.0,3.0,3.0]).unwrap();
        let bad_reshape = x.reshape(vec![2, 2, 2]);
        let rank_error = "New Shape values don't match rank of array"; 
        assert_eq!(bad_reshape, Err(rank_error.to_string()));

        /* size mismatch */ 
        let bad_size = x.reshape(vec![2, 3]); 
        let size_error = "New Shape values don't match size of array";
        assert_eq!(bad_size, Err(size_error.to_string()));  
    }

    #[test]
    fn test_index() {

        /* 2d indexing */ 
        let mut index = 0;
        let n: NDArray<f64> = NDArray::array(vec![2, 3], vec![0.0,0.0,1.0,1.0,2.0,2.0]).unwrap();
        for i in 0..2 {
            for  j in 0..3 {
                assert_eq!(n.index(vec![i, j]).unwrap(), index);
                index += 1; 
            }
        }

        /* 3d indexing */
        index = 0; 
        let m: NDArray<i32> = NDArray::array(vec![2, 2, 2], vec![0,0,1,1,2,2,3,3]).unwrap();
         for i in 0..2 {
            for  j in 0..2 {
                for k in 0..2 {
                    assert_eq!(m.index(vec![i, j, k]).unwrap(), index);
                    index += 1; 
                }
            }
        }

        /* expected indexing error */
        let o: NDArray<i32> = NDArray::array(vec![2, 3], vec![0,0,1,1,2,2]).unwrap();
        let index_bound = o.index(vec![3, 3]);
        let index_error = "Index out of bounds";
        assert_eq!(index_bound, Err(index_error.to_string()));

        /* expected rank error */ 
        let rank_bound = o.index(vec![0, 0, 1]);
        let rank_error = "Indexing doesn't match rank of ndarray"; 
        assert_eq!(rank_bound, Err(rank_error.to_string()));

    }

    #[test]
    fn test_get_indices() {

        /* 2d indexing */ 
        let mut index = 0;
        let n: NDArray<i32> = NDArray::array(vec![2, 3], vec![0,0,1,1,2,2]).unwrap();
        for i in 0..2 {
            for  j in 0..3 {
                assert_eq!(n.indices(index).unwrap(), vec![i, j]);
                index += 1; 
            }
        }

        index = 0; 
        let m: NDArray<i32> = NDArray::array(vec![2, 2, 2], vec![0,0,1,1,2,2,3,3]).unwrap();
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    assert_eq!(m.indices(index).unwrap(), vec![i, j, k]);
                    index += 1; 
                }

            }
        }

        let o: NDArray<i32> = NDArray::array(vec![2, 3], vec![0,0,1,1,2,2]).unwrap();
        let size_bound = o.indices(8);
        let size_error = "Index out of bounds";
        assert_eq!(size_bound, Err(size_error.to_string()));
    }

    #[test]
    fn test_set_ndarray() {

        /* set 2d array */
        let mut index = 0; 
        let mut n: NDArray<i32> = NDArray::array(vec![2, 3], vec![0,0,1,1,2,2]).unwrap();
        for i in 0..2 {
            for  j in 0..3 {
                let _ = n.set(vec![i, j], index);
                index += 1; 
            }
        }

        let n_values = n.values();
        let expected_n : Vec<i32> = vec![0,1,2,3,4,5];
        assert_eq!(n_values, &expected_n);

        /* set 3d array */
        let mut index = 0; 
        let mut n: NDArray<i32> = NDArray::array(vec![2, 2, 2], vec![0,0,1,1,2,2,3,3]).unwrap();
        for i in 0..2 {
            for j in 0..2  {
                for k in 0..2 {
                    let _ = n.set(vec![i, j, k], index);
                    index += 1; 
                }
            }
        }
    }


    #[test]
    fn test_rows_cols_ndarray() {

        /* set 2d array */
        let x: NDArray<f64> = NDArray::array(vec![4, 2], vec![0.0,0.0,0.0,1.0,1.0,1.0,0.0,0.0]).unwrap();
        let y: NDArray<f64> = NDArray::array(vec![2, 3], vec![1.0,1.0,1.0,2.0,2.0,2.0]).unwrap();

        /* get rows of x */ 
        let y_row_0: Vec<f64> = y.rows(0).unwrap(); 
        let y_row_1: Vec<f64> = y.rows(1).unwrap();
        let y_col_0: Vec<f64> = y.cols(0).unwrap(); 
        let y_col_1: Vec<f64> = y.cols(1).unwrap();

        let x_row_0: Vec<f64> = x.rows(0).unwrap(); 
        let x_row_1: Vec<f64> = x.rows(1).unwrap();
        let x_col_0: Vec<f64> = x.cols(0).unwrap(); 
        let x_col_1: Vec<f64> = x.cols(1).unwrap();

        /* validate */ 
        assert_eq!(y_row_0, vec![1.0,1.0,1.0]);
        assert_eq!(y_row_1, vec![2.0,2.0,2.0]); 
        assert_eq!(y_col_0, vec![1.0,2.0]);
        assert_eq!(y_col_1, vec![1.0,2.0]);

        assert_eq!(x_col_0, vec![0.0,0.0,1.0,0.0]);
        assert_eq!(x_col_1, vec![0.0,1.0,1.0,0.0]);
        assert_eq!(x_row_0, vec![0.0,0.0]);
        assert_eq!(x_row_1, vec![0.0,1.0]);

    }

}


#[cfg(test)]
mod ops {

    use crate::structures::ndarray::NDArray;
    use crate::structures::ops::*;

    #[test]
    fn test_save_load_ndarray() {

        /* set 2d array */
        let n: NDArray<f64> = NDArray::array(vec![2, 3], vec![0.0,0.0,1.0,1.0,2.0,2.0]).unwrap();
        let _ = n.save("data/testfile");

        /* load from saved ndarray */ 
        let loaded: NDArray<f64> = NDArray::load("data/testfile").unwrap();
        let shape = loaded.shape();
        let rank = loaded.rank();
        let values = loaded.values();

        /* expected attributes */
        let expected_shape : Vec<usize> = vec![2, 3];
        let expected_size = 6; 
        let expected_vals = vec![0.0,0.0,1.0,1.0,2.0,2.0];

        assert_eq!(shape, &expected_shape);
        assert_eq!(values.len(), expected_size);
        assert_eq!(values, &expected_vals);
        assert_eq!(rank, 2);

    }

    #[test]
    fn test_add_ndarray() {

        /* set 2d array */
        let x: NDArray<f64> = NDArray::array(vec![2, 3], vec![0.0,0.0,1.0,1.0,2.0,2.0]).unwrap();
        let y: NDArray<f64> = NDArray::array(vec![2, 3], vec![0.0,0.0,1.0,1.0,2.0,2.0]).unwrap();

        let result : NDArray<f64> = x.add(y).unwrap(); 
        let expected_vals = vec![0.0,0.0,2.0,2.0,4.0,4.0];
        let expected_shape = vec![2, 3]; 

        assert_eq!(result.rank(), 2); 
        assert_eq!(result.values(), &expected_vals);
        assert_eq!(result.values().len(), 6);
        assert_eq!(result.shape(), &expected_shape);
        
        /* failure case */
        let z: NDArray<f64> = NDArray::array(vec![2, 3], vec![0.0,0.0,1.0,1.0,2.0,2.0]).unwrap();
        let o: NDArray<f64> = NDArray::array(vec![2, 2], vec![0.0,0.0,1.0,1.0]).unwrap();
        let result: Result<NDArray<f64>, String> = o.add(z); // catch the error
        match result {
            Ok(_) => println!("This should fail"), 
            Err(err) => {
                assert_eq!(err, "Add: Size mismatch for arrays"); 
            }
        }

        let a: NDArray<f64> = NDArray::array(vec![2, 2, 2], vec![0.0,0.0,1.0,1.0,2.0,2.0,3.0,3.0]).unwrap();
        let b: NDArray<f64> = NDArray::array(vec![2, 4], vec![0.0,0.0,1.0,1.0,2.0,2.0,3.0,3.0]).unwrap();
        let rank_mismatch: Result<NDArray<f64>, String> = a.add(b); 
        match rank_mismatch {
            Ok(_) => println!("Fail due to rank mismatch"),
            Err(err) => {
                assert_eq!(err, "Add: Rank Mismatch"); 
            }
        }

    }


    #[test]
    fn test_subtract_ndarray() {

        /* set 2d array */
        let x: NDArray<f64> = NDArray::array(vec![2, 3], vec![0.0,0.0,1.0,1.0,2.0,2.0]).unwrap();
        let y: NDArray<f64> = NDArray::array(vec![2, 3], vec![0.0,0.0,1.0,1.0,2.0,2.0]).unwrap();

        let result : NDArray<f64> = x.subtract(y).unwrap(); 
        let expected_vals = vec![0.0,0.0,0.0,0.0,0.0,0.0];
        let expected_shape = vec![2, 3]; 

        assert_eq!(result.rank(), 2); 
        assert_eq!(result.values(), &expected_vals);
        assert_eq!(result.values().len(), 6);
        assert_eq!(result.shape(), &expected_shape);
        
        /* failure case */
        let z: NDArray<f64> = NDArray::array(vec![2, 3], vec![0.0,0.0,1.0,1.0,2.0,2.0]).unwrap();
        let o: NDArray<f64> = NDArray::array(vec![2, 2], vec![0.0,0.0,1.0,1.0]).unwrap();
        let result: Result<NDArray<f64>, String> = o.subtract(z); // catch the error
        match result {
            Ok(_) => println!("This should fail"), 
            Err(err) => {
                assert_eq!(err, "Subtract: Size mismatch for arrays"); 
            }
        }

        let a: NDArray<f64> = NDArray::array(vec![2, 2, 2], vec![0.0,0.0,1.0,1.0,2.0,2.0,3.0,3.0]).unwrap();
        let b: NDArray<f64> = NDArray::array(vec![2, 4], vec![0.0,0.0,1.0,1.0,2.0,2.0,3.0,3.0]).unwrap();
        let rank_mismatch: Result<NDArray<f64>, String> = a.subtract(b); 
        match rank_mismatch {
            Ok(_) => println!("Fail due to rank mismatch"),
            Err(err) => {
                assert_eq!(err, "Subtract: Rank Mismatch"); 
            }
        }
        
    }


    #[test]
    fn test_dot_ndarray() {

        /* set 2d array */
        let a: NDArray<f64> = NDArray::array(vec![4, 3], vec![0.0,0.0,1.0,0.0,1.0,2.0,1.0,1.0,3.0,0.0,0.0,4.0]).unwrap();
        let b: NDArray<f64> = NDArray::array(vec![3, 3], vec![1.0,1.0,1.0,2.0,2.0,2.0,3.0,3.0,3.0]).unwrap();

        let result : NDArray<f64> = a.dot(b).unwrap(); 
        let expected_vals = vec![3.0,3.0,3.0,8.0,8.0,8.0,12.0,12.0,12.0,12.0,12.0,12.0];
        let expected_shape = vec![4, 3];
        
        assert_eq!(result.values(), &expected_vals);
        assert_eq!(result.shape(), &expected_shape);
        assert_eq!(result.values().len(), 12); 
        assert_eq!(result.rank(), 2);
        
        /* set 2d array */
        let x: NDArray<f64> = NDArray::array(vec![4, 2], vec![0.0,0.0,0.0,1.0,1.0,1.0,0.0,0.0]).unwrap();
        let y: NDArray<f64> = NDArray::array(vec![2, 3], vec![1.0,1.0,1.0,2.0,2.0,2.0]).unwrap();

        let result : NDArray<f64> = x.dot(y).unwrap(); 
        let expected_vals = vec![0.0,0.0,0.0,2.0,2.0,2.0,3.0,3.0,3.0, 0.0, 0.0, 0.0];
        let expected_shape = vec![4, 3];
        

        assert_eq!(result.values(), &expected_vals);
        assert_eq!(result.shape(), &expected_shape);
        assert_eq!(result.values().len(), 12); 
        assert_eq!(result.rank(), 2);

            
        /* failure case */
        let z: NDArray<f64> = NDArray::array(vec![2,2,2], vec![0.0,0.0,1.0,1.0,2.0,2.0,3.0,3.0]).unwrap();
        let o: NDArray<f64> = NDArray::array(vec![2, 2], vec![0.0,0.0,1.0,1.0]).unwrap();
        let result: Result<NDArray<f64>, String> = o.dot(z); // catch the error
        match result {
            Ok(_) => println!("This should fail"), 
            Err(err) => {
                assert_eq!(err, "Dot: Rank Mismatch"); 
            }
        }


        let m: NDArray<f64> = NDArray::array(vec![2, 4], vec![0.0,0.0,0.0,1.0,1.0,1.0,0.0,0.0]).unwrap();
        let p: NDArray<f64> = NDArray::array(vec![2, 3], vec![1.0,1.0,1.0,2.0,2.0,2.0]).unwrap();
        let rank_mismatch: Result<NDArray<f64>, String> = m.dot(p); 
        match rank_mismatch {
            Ok(_) => println!("Fail due to rank mismatch"),
            Err(err) => {
                assert_eq!(err, "Dot: Rows must equal columns"); 
            }
        }
        
    }


    #[test]
    fn test_scale_add_ndarray() {

        /* set 2d array */
        let x: NDArray<f64> = NDArray::array(vec![2, 3], vec![0.0,0.0,1.0,1.0,2.0,2.0]).unwrap();
        let y: NDArray<f64> = NDArray::array(vec![1, 3], vec![1.0,1.0,1.0]).unwrap();

        let result : NDArray<f64> = x.scale_add(y).unwrap(); 
        let expected_vals = vec![1.0,1.0,2.0,2.0,3.0,3.0];
        let expected_shape = vec![2, 3]; 

        assert_eq!(result.rank(), 2); 
        assert_eq!(result.values(), &expected_vals);
        assert_eq!(result.values().len(), 6);
        assert_eq!(result.shape(), &expected_shape);
        
        /* set 2d array */
        let a = NDArray::array(vec![4, 3], vec![0.0,0.0,0.0,2.0,2.0,2.0,2.0,2.0,2.0,4.0,4.0,4.0]).unwrap();
        let b = NDArray::array(vec![1, 3], vec![1.0,1.0,1.0]).unwrap();

        let result_two : NDArray<f64> = a.scale_add(b).unwrap(); 
        let expected_vals_two = vec![1.0,1.0,1.0,3.0,3.0,3.0,3.0,3.0,3.0,5.0,5.0,5.0];
        let expected_shape_two = vec![4, 3]; 

        assert_eq!(result_two.rank(), 2); 
        assert_eq!(result_two.values(), &expected_vals_two);
        assert_eq!(result_two.values().len(), 12);
        assert_eq!(result_two.shape(), &expected_shape_two);

        /* failure case */
        let o = NDArray::array(vec![4, 3], vec![0.0,0.0,0.0,2.0,2.0,2.0,2.0,2.0,2.0,4.0,4.0,4.0]).unwrap();
        let n = NDArray::array(vec![2, 2], vec![1.0,1.0,1.0,1.0]).unwrap();
        let result_bad: Result<NDArray<f64>, String> = o.scale_add(n);
        match result_bad {
            Ok(_) => println!("Fail due to dimension mismatch"),
            Err(err) => {
                assert_eq!(err, "Scale add must have a vector dimension (1, N)"); 
            }
        }

    }


    #[test]
    fn test_transpose_ndarray() {

        /* set 2d array */
        let x = NDArray::array(vec![4, 3], vec![1.0,2.0,3.0,1.0,2.0,3.0,0.0,0.0,2.0,0.0,0.0,0.0]).unwrap();
        let x_transpose = x.transpose().unwrap();
        let expected_shape : Vec<usize> = vec![3, 4];
        let expected_vals = vec![1.0,1.0,0.0,0.0,2.0,2.0,0.0,0.0,3.0,3.0,2.0,0.0];

        assert_eq!(x_transpose.shape(), &expected_shape);
        assert_eq!(x_transpose.rank(), 2);
        assert_eq!(x_transpose.size(), 12);
        assert_eq!(x_transpose.values(), &expected_vals);

        /* failure case */ 
        let y = NDArray::array(vec![2, 2, 2], vec![1.0,1.0,2.0,2.0,3.0,3.0,4.0,4.0]).unwrap();
        let expected_shape_two : Vec<usize> = vec![2, 2, 2];
        let expected_vals_two = vec![1.0,1.0,2.0,2.0,3.0,3.0,4.0,4.0];

        assert_eq!(y.rank(), 3); 
        assert_eq!(y.values(), &expected_vals_two);
        assert_eq!(y.values().len(), 8);
        assert_eq!(y.shape(), &expected_shape_two);

        let rank_mismatch = y.transpose();
        match rank_mismatch {
            Ok(_) => println!("Should fail due to rank mismatch"),
            Err(err) => {
                assert_eq!(err, "Transpose must contain on rank 2 values"); 
            }
        }

    }

    #[test]
    fn test_permute_ndarray() {

        /* set 2d array */
        let x = NDArray::array(vec![4, 3], vec![1.0,2.0,3.0,1.0,2.0,3.0,0.0,0.0,2.0,0.0,0.0,0.0]).unwrap();
        let expected_shape : Vec<usize> = vec![3, 4];
        let expected_vals = vec![1.0,1.0,0.0,0.0,2.0,2.0,0.0,0.0,3.0,3.0,2.0,0.0];
        let result = x.permute(vec![1, 0]).unwrap();

        assert_eq!(result.shape(), &expected_shape);
        assert_eq!(result.rank(), 2);
        assert_eq!(result.size(), 12);
        assert_eq!(result.values(), &expected_vals);

        /* rank mismatch case */ 
        let y = NDArray::array(vec![4, 3], vec![1.0,2.0,3.0,1.0,2.0,3.0,0.0,0.0,2.0,0.0,0.0,0.0]).unwrap(); 
        let rank_mismatch = y.permute(vec![1]);
        match rank_mismatch {
            Ok(_) => println!("Should error out"),
            Err(err) => {
                assert_eq!(err, "Indice order must be same length as rank"); 
            }
        }

    }
}