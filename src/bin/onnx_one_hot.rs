// Generated from ONNX "tests/one_hot/one_hot.onnx" by burn-import
use burn::record::FullPrecisionSettings;
use burn::record::Recorder;
use burn::tensor::Int;
use burn::{
    module::Module,
    tensor::{backend::Backend, Tensor},
};

#[derive(Module, Debug)]
pub struct Model<B: Backend> {
    constant2: burn::module::Param<Tensor<B, 1, Int>>,
    phantom: core::marker::PhantomData<B>,
    device: burn::module::Ignored<B::Device>,
}

impl<B: Backend> Default for Model<B> {
    fn default() -> Self {
        Self :: from_file ("/Users/tiruka/Desktop/oss-fork/burn/target/debug/build/onnx-tests-8303501eab20e2dc/out/model/one_hot" , & Default :: default ())
    }
}

impl<B: Backend> Model<B> {
    pub fn from_file(file: &str, device: &B::Device) -> Self {
        let record = burn::record::NamedMpkFileRecorder::<FullPrecisionSettings>::new()
            .load(file.into(), device)
            .expect("Record file to exist.");
        Self::new(device).load_record(record)
    }
}

impl<B: Backend> Model<B> {
    #[allow(unused_variables)]
    pub fn new(device: &B::Device) -> Self {
        let constant2: burn::module::Param<Tensor<B, 1, Int>> = burn::nn::Initializer::Zeros
            .init([2], device)
            .set_require_grad(false);
        Self {
            constant2,
            phantom: core::marker::PhantomData,
            device: burn::module::Ignored(device.clone()),
        }
    }

    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(&self, input1: Tensor<B, 1, Int>) -> Tensor<B, 1> {
        let constant1_out1: i64 = 6i64;
        let constant2_out1 = self.constant2.val();
        let mut onehot1_out1 = input1.one_hot(1usize);
        onehot1_out1 = onehot1_out1 * (0i64 - 0i64) + 0i64;
        let cast1_out1 = onehot1_out1.float();
        let constant3_out1: f32 = 1f32;
        let mul1_out1 = cast1_out1.mul_scalar(constant3_out1);
        let constant4_out1: f32 = 0f32;
        let add1_out1 = mul1_out1.add_scalar(constant4_out1);
        add1_out1
    }
}
