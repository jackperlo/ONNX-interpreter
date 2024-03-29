use ndarray::{Array, Array4};

pub fn relu(x: &Array4<f32>) -> Array4<f32> {
  x.map(|&val| val.max(0.0))
}

#[allow(dead_code)]
pub fn test_relu(){
  let input = Array::from_shape_vec(
    (1, 1, 7, 5),
    vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, -17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0, 31.0, 32.0, 33.0, 34.0]
  )
    .unwrap();

  let output = Array::from_shape_vec(
    (1, 1, 7, 5),
    vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 0.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0, 31.0, 32.0, 33.0, 34.0]
  )
    .unwrap();

  println!("{:?}", relu(&input));
  println!("{:?}", output);
}