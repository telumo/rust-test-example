use std::{
    error,
    fs::{read_to_string, write},
    path::Path,
    result,
};

type TResult<T> = result::Result<T, TError>;
type TError = Box<dyn error::Error>;

// 1. ファイルを文字列として読み込む
fn read_file(p: &str) -> TResult<String> {
    read_to_string(p).map_err(|e| e.into())
}

// 2. 文字列を数値のベクトルに変換する
fn split_numbers(s: &String) -> TResult<Vec<usize>> {
    s.split_whitespace()
        .map(|x| x.parse::<usize>().map_err(|e| e.into()))
        .collect()
}

// 3. 全ての数字を合計する
fn add_numbers(v: Vec<usize>) -> usize {
    v.iter().fold(0, |mut sum, &x| {
        sum += x;
        sum
    })
}

// 4. 同じファイルに合計を書き込む
fn write_numbers(n: usize, p: &str) -> TResult<()> {
    let path = Path::new(p);
    let res = read_to_string(&path.display().to_string())?;

    write(path, format!("{}\n{}", res, n))?;

    Ok(())
}


fn main() -> TResult<()> {
    let path = "data/numbers.txt";
    let res = read_to_string(&path);

    match res {
        Ok(s) => {
            let vec = split_numbers(&s)?;
            println!("vector: {:?}", &vec);
            let sum = add_numbers(vec);
            println!("Sum: {:?}", &sum);
            write_numbers(sum, path)?;
        }
        Err(_) => {}
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup_test_write() -> TResult<()> {
        write(Path::new("test_data/test_two.txt"), String::from("4\n6"))?;
        Ok(())
    }

    #[test]
    fn test_read_file() {
        let res = read_file("test_data/test_one.txt");
        assert!(res.is_ok());

        if let Ok(s) = res {
            assert_eq!(s, "3\n5");
        }
    }

    #[test]
    fn test_split_numbers() {
        let res = split_numbers(&String::from("5\n8"));

        assert!(res.is_ok());

        if let Ok(v) = res {
            assert_eq!(v, vec![5, 8])
        }
    }

    #[test]
    fn test_add_numbers() {
        let sum = add_numbers(vec![3, 5]);
        assert_eq!(sum, 8);
        let sum = add_numbers(vec![1]);
        assert_eq!(sum, 1);
        let sum = add_numbers(vec![4, 7, 2]);
        assert_eq!(sum, 13);
        let sum = add_numbers(vec![]);
        assert_eq!(sum, 0);
    }

    #[test]
    fn test_write_numbers() {
        let res = setup_test_write();
        assert!(res.is_ok());

        let res = write_numbers(10, "test_data/test_two.txt");
        assert!(res.is_ok());

        let res = read_to_string("test_data/test_two.txt");
        if let Ok(s) = res {
            assert_eq!(s, "4\n6\n10");
        }
    }
}
