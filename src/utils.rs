use rand::distributions::Standard;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg; // 擬似乱数生成器

pub fn new_u32_vec(n: usize) -> Vec<u32> {
    // RNGを初期化する。再現性を持たせるため毎回同じシード値を使う
    let rng = Pcg64Mcg::from_seed([0; 16]);

    // rng.sample_iter() は乱数を無限に生成するイテレータを返す
    // take(n) は元のイテレータから最初のn要素だけを取り出すイテレータを返す
    // collect() はイテレータから値を収集して、ベクタやハッシュマップのようなコレクションに格納する
    rng.sample_iter(&Standard).take(n).collect()
}

pub fn is_sorted_ascending<T: Ord>(x: &[T]) -> bool {
    x.windows(2).all(|pair| pair[0] <= pair[1])
}

pub fn is_sorted_descending<T: Ord>(x: &[T]) -> bool {
    x.windows(2).all(|pair| pair[0] >= pair[1])
}
