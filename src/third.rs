use super::SortOrder;

pub fn sort<T: Ord>(x: &mut [T], order: &SortOrder) -> Result<(), String> {
    if x.len().is_power_of_two() {
        match *order {
            SortOrder::Ascending => do_sort(x, true),
            SortOrder::Descending => do_sort(x, false),
        }
        Ok(())
    } else {
        Err(format!(
            "The length of x is not a power of two. (x.len() : {})",
            x.len()
        ))
    }
}

fn do_sort<T: Ord>(x: &mut [T], up: bool) {
    // スライスの参照を受け取っているので、xの長さが1以下であれば何も処理を行わない
    if x.len() > 1 {
        let mid_point = x.len() / 2;
        do_sort(&mut x[..mid_point], true);
        do_sort(&mut x[mid_point..], false);
        sub_sort(x, up);
    }
}

fn sub_sort<T: Ord>(x: &mut [T], up: bool) {
    if x.len() > 1 {
        compare_and_swap(x, up);
        let mid_point = x.len() / 2;
        sub_sort(&mut x[..mid_point], up);
        sub_sort(&mut x[mid_point..], up);
    }
}

fn compare_and_swap<T: Ord>(x: &mut [T], up: bool) {
    let mid_point = x.len() / 2;
    for i in 0..mid_point {
        if (x[i] > x[mid_point + i]) == up {
            // 要素を交換する
            x.swap(i, mid_point + i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sort;
    use crate::SortOrder::*;

    // 構造体Studentを定義
    struct Student {
        first_name: String, // 名前フィールド
        last_name: String,  // 苗字フィールド
        age: u8,            // 年齢フィールド
    }

    // implブロックを使うと対象の型に関連関数やメソッドを実装できる
    impl Student {
        // 関連関数newを実装
        fn new(first_name: &str, last_name: &str, age: u8) -> Self {
            // 構造体Studentを初期化して返す
            Self {
                first_name: first_name.to_string(),
                last_name: last_name.to_string(),
                age, // フィールドと変数名が同じ名前のときは、この省略形でかける
            }
        }
    }

    #[test]
    fn sort_to_fail() {
        let mut x = [3, 11, 7];
        assert!(sort(&mut x, &Ascending).is_err());
    }

    #[test]
    fn sort_u32_ascending() {
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
        assert_eq!(sort(&mut x, &Ascending), Ok(()));

        assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn sort_u32_descending() {
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
        assert_eq!(sort(&mut x, &Descending), Ok(()));

        assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
    }

    #[test]
    fn sort_str_ascending() {
        let mut x = vec![
            "Rust",
            "is",
            "fast",
            "and",
            "memory-efficient",
            "with",
            "no",
            "GC",
        ];
        assert_eq!(sort(&mut x, &Ascending), Ok(()));
        assert_eq!(
            x,
            vec![
                "GC",
                "Rust",
                "and",
                "fast",
                "is",
                "memory-efficient",
                "no",
                "with"
            ]
        );
    }

    #[test]
    fn sort_str_descending() {
        let mut x = vec![
            "Rust",
            "is",
            "fast",
            "and",
            "memory-efficient",
            "with",
            "no",
            "GC",
        ];
        assert_eq!(sort(&mut x, &Descending), Ok(()));
        assert_eq!(
            x,
            vec![
                "with",
                "no",
                "memory-efficient",
                "is",
                "fast",
                "and",
                "Rust",
                "GC"
            ]
        );
    }

    #[test]
    fn sort_students_by_age_ascending() {
        let taro = Student::new("Taro", "Yamada", 16);
        let hanako = Student::new("Hanako", "Yamada", 14);
        let kyoko = Student::new("Kyoko", "Ito", 15);
        let ryosuke = Student::new("Ryosuke", "Hayashi", 17);

        // ソート対象のベクタ
        let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];

        // ソートの期待値
        let expected = vec![&hanako, &kyoko, &taro, &ryosuke];

        assert_eq!(sort_by(&mut x, &|a, b| a.age.cmp(&b.age)), Ok(()));

        assert_eq!(x, expected);
    }
}
