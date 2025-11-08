
pub fn getFloor(input : String) -> (i32, String) {
    let mut highFloor : i32 = 0;
    let mut lowFloor : i32 = 0;
    let comparator : char = '(';

    input.chars().for_each(|input : char|
        if (input.eq(&comparator)) { highFloor += 1;}
        else {lowFloor += 1;}
    );
    return (highFloor-lowFloor, input);
}

pub fn getFirstBasementValue(input : String) -> i32 {
    let mut highFloor : i32 = 0;
    let mut lowFloor : i32 = 0;
    let comparator : char = '(';

    for (i, val) in input.chars().enumerate() {
        if (val.eq(&comparator)) { highFloor += 1;}
        else {lowFloor += 1;}

        if(highFloor - lowFloor == -1) {
            return (i as i32 + 1);
            //task is based on 1-indexing rather than 0 index
        }
    }
    return -1;
}