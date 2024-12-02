use burn::backend::NdArray;
use burn::tensor::{Int, Tensor};
type Backend = NdArray;
fn main() {
    let device = Default::default();
    // one hot from tensor/api/int.rs
    let original_data: Tensor<Backend, 1, Int> = Tensor::from_ints([0, 1, 2, 3], &device);
    // let original_data: Tensor<Backend, 1, Int> = Tensor::from_ints([0, 1, 2, 0, 4], &device);

    println!("original_data: {}\n", original_data.to_data());
    // [0, 1, 2, 3]

    // 本来動くべきコード torch.nn.functional.one_hotに近い。
    // let one_hot = original_data.one_hot(4);
    // println!("{}", one_hot);
    // 期待するのは、下記
    //[[1, 0, 0, 0],
    // [0, 1, 0, 0],
    // [0, 0, 1, 0],
    // [0, 0, 0, 1]]
    // one_hotの中身を持ってきて、検証する。
    let [num_samples] = original_data.dims();
    println!("#num_samples: {}\n", &num_samples); // 4
    let indices: Tensor<Backend, 2, Int> = original_data.unsqueeze();
    println!("#indices: {}\n", indices); // [[0, 1, 2, 3]],
    let values: Tensor<Backend, 2, Int> = indices.ones_like();
    println!("#values: {}\n", values); // [[1, 1, 1, 1]],
    let zeros: Tensor<Backend, 2, Int> =
        Tensor::zeros([num_samples, num_samples], &indices.device());
    println!("#zeros: {}\n", zeros);
    // [[0, 0, 0, 0],
    // [0, 0, 0, 0],
    // [0, 0, 0, 0],
    // [0, 0, 0, 0]]

    // おそらく、下記のscatterがbug
    // original let r = zeros.scatter(1, indices, values);
    // dimを0にしたら、うまくいった。dim = 0のときは、行方向に入れ替えていくので、納得である。
    // https://github.com/tracel-ai/burn/pull/2501/files で修正が入った（やり方は逆だが本質は同じ）
    let r = zeros.scatter(0, indices, values);
    println!("#result: {}", r);

    // one hot from tensor/api/float.rs こっちは動く。
    // let one_hot = Tensor::<Backend, 1>::one_hot(2, 10, &device);
    // println!("#float one hot: {}", one_hot);
    // [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
}
