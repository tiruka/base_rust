use burn::backend::NdArray;
use burn::tensor::{Int, Tensor};
type Backend = NdArray;

fn main() {
    let device = Default::default();
    // one hot from tensor/api/int.rs
    let original_data: Tensor<Backend, 1, Int> = Tensor::from_ints([0, 1, 2, 3], &device);
    println!("original_data: {}\n", original_data.to_data());
    // [0, 1, 2, 3]

    // 本来動くべきコード torch.nn.functional.one_hotに近い。
    // let one_hot = original_data.one_hot(4);
    // println!("{}", one_hot.to_data());
    // [[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]]
    // one_hotの中身を持ってきて、検証する。
    let [num_samples] = original_data.dims();
    println!("#num_samples: {}\n", &num_samples);
    let indices: Tensor<Backend, 2, Int> = original_data.unsqueeze();
    println!("#indices: {}\n", indices);
    let values: Tensor<Backend, 2, Int> = indices.ones_like();
    println!("#values: {}\n", values);
    let zeros: Tensor<Backend, 2, Int> = Tensor::zeros([4, 4], &indices.device());
    println!("#zeros: {}\n", zeros);
    // let r = zeros.scatter(1, indices, values);
    // println!("#result: {}", r.to_data());

    // one hot from tensor/api/float.rs こっちは動く。
    // let one_hot = Tensor::<Backend, 1>::one_hot(2, 10, &device);
    // println!("{}", one_hot.to_data());
    // [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
}
