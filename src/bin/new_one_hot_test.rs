#![allow(unused)]
use burn::backend::NdArray;
use burn::tensor::{Int, Tensor};
type Backend = NdArray;
type B = Backend;
fn main() {
    let device = Default::default();
    let expected: Tensor<B, 2, Int> =
        Tensor::from_ints([[5, 0, 0], [0, 0, 5], [0, 5, 0], [0, 0, 0]], &device);

    let indices: Tensor<B, 2, Int> = Tensor::from_ints([[0, 2], [1, -1]], &device);

    let depth = 3;
    let on_value = 5;
    let off_value = 0;
    let axis = -1;

    // pesude one hot function.
    let shape = indices.shape();
    println!("shape: {:?}", shape);

    // Create a zero tensor and scatter on_value
    // let output = off_tensor.scatter(actual_axis, indices_expanded, on_tensor);
    // output.into_data().assert_eq(&expected.into_data(), false);
}
