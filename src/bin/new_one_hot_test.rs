use burn::backend::NdArray;
use burn::tensor::{Int, Tensor};
type Backend = NdArray;
type B = Backend;
fn main() {
    let device = Default::default();
    let indices: Tensor<B, 2, Int> = Tensor::from_ints([[0, 2], [1, -1]], &device);
    let expected: Tensor<B, 2, Int> =
        Tensor::from_ints([[5, 0, 0], [0, 0, 5], [0, 5, 0], [0, 0, 0]], &device);
    let output: Tensor<B, 2, Int> =
        Tensor::from_ints([[5, 0, 0], [0, 0, 5], [0, 5, 0], [0, 0, 0]], &device);
    output.into_data().assert_eq(&expected.into_data(), false);
}
