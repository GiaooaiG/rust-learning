fn main() {
    let input = [1,1,23,-10,5,-50,24,-100];
    println!("{}",max_sum(&input));
}
fn max_sum(input :&[i32]) -> i32{
    if input.len() == 1 {
        if input[0] < 0 {
            return 0;
        } else {
            return input[0];
        }
    }
    let max_sum_left:i32 =max_sum(&input[..input.len()/2]);
    let max_sum_right:i32 =max_sum(&input[input.len()/2..]);
    let mut max_sum_left_broader:i32=0;
    let mut max_sum_right_broader:i32=0;
    let mut sum_left_broader:i32 = 0;
    let mut sum_right_broader:i32 = 0;
    for i in input[..input.len()/2].iter().rev() {
        sum_left_broader += i;
        if sum_left_broader > max_sum_left_broader{
            max_sum_left_broader = sum_left_broader;
        }
    }
    for i in input[input.len()/2..].iter() {
        sum_right_broader += i;
        if sum_right_broader > max_sum_right_broader{
            max_sum_right_broader = sum_right_broader;
        }
    }
    let max_sum_broader = max_sum_left_broader + max_sum_right_broader;

    // compare 3
    if max_sum_broader > max_sum_left {
        if max_sum_broader > max_sum_right {
            return max_sum_broader;
        }else {
            return max_sum_right;
        }
    }else{
        if max_sum_left > max_sum_right{
            return max_sum_left;
        }else {
            return max_sum_right;
        }
    }

}
