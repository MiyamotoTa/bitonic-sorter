// pubはこのsort関数が他のモジュールからアクセスできることを示す
// 引数xの型 &mut [u32] について
//  &は値をポインタ経由で借用する
//  mutは値が変更可能であることを示す
//  u32型は32ビット符号なし整数
//  [u32]型はu32のスライス
pub fn sort(x: &mut [u32], up: bool) {
    // スライスの参照を受け取っているので、xの長さが1以下であれば何も処理を行わない
    if x.len() > 1 {
        let mid_point = x.len() / 2;
        sort(&mut x[..mid_point], true);
        sort(&mut x[mid_point..], false);
        sub_sort(x, up);
    }
}

fn sub_sort(x: &mut [u32], up: bool) {
    if x.len() > 1 {
        compare_and_swap(x, up);
        let mid_point = x.len() / 2;
        sub_sort(&mut x[..mid_point], up);
        sub_sort(&mut x[mid_point..], up);
    }
}

fn compare_and_swap(x: &mut [u32], up: bool) {
    let mid_point = x.len() / 2;
    for i in 0..mid_point {
        if (x[i] > x[mid_point + i]) == up {
            // 要素を交換する
            x.swap(i, mid_point + i);
        }
    }
}
