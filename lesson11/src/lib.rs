pub fn add(left: usize, right: usize) -> usize {
    left + right
}



struct Rectangle {
    width:i32,
    height:i32,
}

impl Rectangle {
    fn can_hold(&self, other:&Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}


#[cfg(test)]

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // #[test]
    // fn failed_test() {
    //     panic!("Test failed!");
    // }

// use super::*;

#[test]
fn larger_rec_can_hold_smaller() {
    let large_rec = Rectangle{
        width:3,
        height:4,
    };

    let small_rec = Rectangle{
        width:2,
        height:1,
    };

    assert!(large_rec.can_hold(&small_rec));
}