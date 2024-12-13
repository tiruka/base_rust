#![allow(unused)]
use std::result;
use std::slice::RSplit;

use burn::backend::NdArray;
use burn::module::Module;
use burn::serde::de;
use burn::tensor::{Float, Int, Tensor};
type Backend = NdArray;
type B = Backend;
fn main() {
    let device = Default::default();
    let expected: Tensor<B, 2, Float> = Tensor::from_floats(
        [
            [3.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
            [1.0, 1.0, 1.0, 3.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
            [1.0, 1.0, 3.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
        ],
        &device,
    );

    let indices: Tensor<B, 1, Int> = Tensor::from_ints([0, -7, -8], &device);
    // indicesの条件が当てはまらない時の動作がまだ。ONNX 9と11で条件が違うが、それにtensorflowも追随していない。0以上で条件を区切っている。
    // おそらく、マイナスの場合は、pythonは普通に後ろからのインデックスになるので、len(list) - negative みたいにする必要がある。
    // negativeの値をrustのscatterが受け入れるかどうか。あと、tensorflowの方も直した方が良さそう。てか直せんのか？

    // println!("original indices\n{:?}\n#######", &indices);

    let depth = 10;
    let on_value = 3.0;
    let off_value = 1.0;
    let mut axis: i64 = 1;

    let mut shape = indices.shape().dims::<1>().to_vec();
    // println!("shape: {:?}\n#######", shape);
    let rank = indices.dims().len();
    let axis = if axis < 0 {
        axis + rank as i64 + 1 // Convert negative axis to positive index
    } else {
        axis
    };
    if axis < 0 || axis > rank as i64 {
        panic!("Axis out of range. Accepted range is [-r-1, r] where r = rank(indices).");
    }
    shape.insert(axis as usize, depth);
    // 条件1: indices >= 0
    let condition1 = indices.clone().greater_elem(-1 * depth as i64).int();

    // 条件2: indices < depth
    let condition2 = indices.clone().lower_elem(depth as i64).int();
    // println!("conditon 1\n{:?}\n#######", &condition1);
    // println!("conditon 2\n{:?}\n#######", &condition2);
    // 論理AND: valid_mask 乗算 (1 * 1 = 1, 他は0), さらにそれを反転させる。1のところは、そのままにしたいので。
    let valid_mask = condition1.mul(condition2).bool().bool_not();
    // println!("valid mask\n{:?}\n#######", &valid_mask);
    let condition3 = indices.clone().lower_elem(0);

    let adjusted_indices = indices
        .clone()
        .mask_fill(indices.clone().lower_elem(0), depth as i64)
        .add(
            indices
                .clone()
                .mask_fill(indices.clone().greater_elem(0), 0),
        );

    println!("#######adjusted indices\n{:?}\n", &adjusted_indices);
    // 0未満、depth以上のデータを排除した、valid indicesを作る
    let valid_indices = adjusted_indices.mask_fill(valid_mask, off_value);
    println!("#######valid indices\n{:?}\n", &valid_indices);

    let indices_unsqueezed = valid_indices.unsqueeze_dim(axis as usize);
    // println!(
    //     "indices_unsqueezed #######\n{:?}\n#######",
    //     &indices_unsqueezed
    // );

    // ここから、outputを作成する 型指定をしているけど、自動でどうにかなるはず。
    let result: Tensor<B, 2, Int> = Tensor::full(shape.clone(), off_value, &device);
    println!("original result #######\n{:?}\n#######", &result);

    let scatter_on_values = Tensor::full(indices_unsqueezed.shape(), on_value, &device);
    let scatter_off_values = Tensor::full(indices_unsqueezed.shape(), off_value, &device);
    let scatter_on_values = scatter_on_values - scatter_off_values;
    let result = result.scatter(axis as usize, indices_unsqueezed.clone(), scatter_on_values);

    println!("####### final result #######\n{:?}\n", &result);
    println!("####### expected #######\n{:?}\n", &expected);

    result.into_data().assert_eq(&expected.into_data(), false);
}
