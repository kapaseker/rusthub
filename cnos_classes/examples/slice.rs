fn main() {
    let arr = [1, 2, 3, 4, 5];
    let a_on1 = &arr[0..2];
    println!("{:?}", a_on1);
    // arr[1] = 12;
    // println!("{:?}", a_on1);
    {
        let arr = vec![];
        println!("max : {:?}", largest_one(&arr));
    }

    {
        let arr = vec![1, 2, 3, 4, 5];
        println!("max : {:?}", largest_one(&arr));
    }

        {
        let arr = vec![10, 2, 30, 4, 5];
        println!("max : {:?}", largest_one(&arr));
    }


    let test_arr = [1,2,3,4,5];
    match test_arr {
        [1,remaining @ ..] => {
            println!("{:?}",remaining);
        }
        _ => {

        }
    }
}

fn largest_one(arr: &[i32]) -> Option<&i32> {
    return if arr.len() > 0 {
        let mut max = &arr[0];

        for i in arr {
            if i > max {
                max = i;
            }
        }

        Some(max)
    } else {
        None
    }
}
