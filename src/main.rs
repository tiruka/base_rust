use burn::backend::Wgpu;
use burn::tensor::{Int, Tensor};
type Backend = Wgpu;

fn main() {
    let device = Default::default();
    let data = [[2., 3.], [4., 5.]];
    let tensor_1 = Tensor::<Backend, 2>::from_data(data, &device);
    let tensor_2 = Tensor::ones_like(&tensor_1);
    println!("{}", tensor_1 + tensor_2);

    // one hot from tensor/api/int.rs
    let indices: Tensor<Backend, 1, Int> = Tensor::from_ints([0, 1, 2, 3], &device);
    let one_hot = indices.one_hot(4);
    println!("{}", one_hot.to_data());
    // [[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]]

    // one hot from tensor/api/float.rs
    let one_hot = Tensor::<Backend, 1>::one_hot(2, 10, &device);
    println!("{}", one_hot.to_data());
    // [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
}
