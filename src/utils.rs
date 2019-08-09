use rand::distributions::Standard;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg; // 擬似乱数生成器

pub fn new_u32_vec(n: usize) -> Vec<u32> {
    // RNGを初期化する。再現性を持たせるため毎回同じシード値を使う
    let mut rng = Pcg64Mcg::from_seed([0; 16]);

    // n個の要素が格納できるようベクタを初期化
    let mut v = Vec::with_capacity(n);

    // 0からn-1までの合計n回、繰り返し乱数を生成し、ベクタに追加する
    // 0からn-1の数列は使わないので _ で破棄
    for _ in 0..n {
        // RNGのsampleメソッドは引数として与えられた分布にしたがう乱数を1つ生成する
        // Standard分布は生成する値が数値型のときは一様分布
        v.push(rng.sample(&Standard));
    }

    v
}
