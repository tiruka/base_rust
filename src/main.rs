use burn::backend::NdArray;
use burn::tensor::{Int, Tensor};
type Backend = NdArray;

fn main() {
    let device = Default::default();
    // one hot from tensor/api/int.rs
    let original_indices: Tensor<Backend, 1, Int> = Tensor::from_ints([0, 1, 2, 3], &device);
    println!("original_indices: {}", original_indices);
    // [0, 1, 2, 3]

    // 本来動くべきコード torch.nn.functional.one_hotに近い。
    // let one_hot = original_indices.one_hot(4);
    // println!("{}", one_hot.to_data());
    // [[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]]
    // one_hotの中身を持ってきて、検証する。
    let [num_samples] = original_indices.dims();
    let indices: Tensor<Backend, 2, Int> = original_indices.unsqueeze();
    let values: Tensor<Backend, 2, Int> = indices.ones_like();
    let zeros: Tensor<Backend, 2, Int> =
        Tensor::zeros([num_samples, num_samples], &indices.device());
    let r = zeros.scatter(1, indices, values);
    println!("{}", r.to_data());

    // one hot from tensor/api/float.rs こっちは動く。
    // let one_hot = Tensor::<Backend, 1>::one_hot(2, 10, &device);
    // println!("{}", one_hot.to_data());
    // [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
}
