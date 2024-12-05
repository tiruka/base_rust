#![allow(unused)]
use std::result;
use std::slice::RSplit;

use burn::backend::NdArray;
use burn::serde::de;
use burn::tensor::{Int, Tensor};
type Backend = NdArray;
type B = Backend;
fn main() {
    let device = Default::default();
    let expected: Tensor<B, 2, Int> =
        Tensor::from_ints([[5, 0, 0], [0, 0, 5], [0, 5, 0], [0, 0, 0]], &device);

    let indices: Tensor<B, 2, Int> = Tensor::from_ints([[0, 2], [1, -1]], &device);
    // println!("original indices\n{:?}\n#######", &indices);

    let depth = 3;
    let on_value = 5;
    let off_value = 0;
    let axis = -1;

    // pesude one hot function.2ではなくDが入る想定
    let mut shape = indices.shape().dims::<2>().to_vec();
    // println!("shape: {:?}\n#######", shape);
    if axis == -1 {
        shape.push(depth);
    } else if axis == 0 {
        shape.insert(0, depth);
    } else {
        panic!("Only axis=-1 or axis=0 are supported.");
    }
    // 条件1: indices >= 0
    let condition1 = indices.clone().greater_equal_elem(0).int();

    // 条件2: indices < depth
    let condition2 = indices.clone().lower_elem(depth as i64).int();
    // println!("conditon 1\n{:?}\n#######", &condition1);
    // println!("conditon 2\n{:?}\n#######", &condition2);
    // 論理AND: valid_mask 乗算 (1 * 1 = 1, 他は0), さらにそれを反転させる。1のところは、そのままにしたいので。
    let valid_mask = condition1.mul(condition2).bool().bool_not();
    // println!("valid mask\n{:?}\n#######", &valid_mask);

    // 0未満、depth以上のデータを排除した、valid indicesを作る
    let valid_indices = indices.mask_fill(valid_mask, 0);
    println!("valid indices\n{:?}\n#######", &valid_indices);

    let dim = if axis == -1 {
        valid_indices.dims().len() // 次元数を取得
    } else if axis == 0 {
        0
    } else {
        panic!("Invalid axis.");
    };
    let indices_unsuqueezed = valid_indices.unsqueeze_dim(dim);
    println!(
        "indices_unsuqueezed #######\n{:?}\n#######",
        &indices_unsuqueezed
    );

    // ここから、outputを作成する 型指定をしているけど、自動でどうにかなるはず。
    let result: Tensor<B, 3, Int> = Tensor::full(shape.clone(), off_value, &device);
    println!("original result #######\n{:?}\n#######", &result);

    let values_shape = result.shape();
    let result = result.scatter(
        dim,
        indices_unsuqueezed,
        Tensor::full(values_shape, on_value, &device),
    );

    println!("final result #######\n{:?}\n#######", &result);

    // Create a zero tensor and scatter on_value
    // let output = off_tensor.scatter(actual_axis, indices_expanded, on_tensor);
    // output.into_data().assert_eq(&expected.into_data(), false);
}
