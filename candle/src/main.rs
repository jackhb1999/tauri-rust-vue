use candle_core::{DType, Device, Tensor};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let device = Device::Cpu;

    let a = Tensor::randn(0f32, 1., (2, 3), &device)?;
    let b = Tensor::randn(0f32, 1., (3, 4), &device)?;

    let c = a.matmul(&b)?;
    println!("{c}");

    let x = Tensor::arange(0., 12., &device)?;
    let shape = x.shape();
    let numel = x.elem_count();
    println!("{:?}", shape);
    println!("{:?}", numel);
    let X = x.reshape((3, ()))?;
    println!("{:?}", X);

    let z = Tensor::zeros((2, 3, 4), DType::F16, &device)?;
    println!("{z}");

    let o = Tensor::ones((2, 3, 4), DType::F16, &device)?;
    println!("{o}");

    let r = Tensor::randn(0., 1., (3, 4), &device)?;
    println!("{r}");

    let f = Tensor::from_vec(vec![1., 2., 3., 4., 5., 6.], (2, 3), &device)?;
    println!("{f}");

    let n = Tensor::new(
        &[[2., 1., 4., 3.], [1., 2., 3., 4.], [4., 3., 2., 1.]],
        &device,
    )?;
    println!("{n}");

    let x = Tensor::new(&[1.0, 2., 4., 8.], &device)?;
    let y = Tensor::new(&[2., 2., 2., 2.], &device)?;
    let sum = &x.add(&y)?;
    println!("{sum}");

    Ok(())
}
