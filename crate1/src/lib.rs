pub fn one()   -> i32 { 1 }
pub fn two()   -> i32 { 2 }
pub fn three() -> i32 { 3 }
pub fn four()  -> i32 { 4 }
pub fn five()  -> i32 { 5 }
pub fn six()   -> i32 { 6 }
pub fn seven() -> i32 { 7 }
pub fn eight() -> i32 { 8 }
pub fn nine()  -> i32 { 9 }
pub fn ten()   -> i32 { 10 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one(){ assert_eq!(one(), 1); }
    #[test]
    fn test_two(){ assert_eq!(two(), 2); }
    #[test]
    fn test_three(){ assert_eq!(three(), 3); }
    #[test]
    fn test_four(){ assert_eq!(four(), 4); }
    #[test]
    fn test_five(){ assert_eq!(five(), 5); }
    #[test]
    fn test_six(){ assert_eq!(six(), 6); }
    #[test]
    fn test_seven(){ assert_eq!(seven(), 7); }
    #[test]
    fn test_eight(){ assert_eq!(eight(), 8); }
    #[test]
    fn test_nine(){ assert_eq!(nine(), 9); }
    #[test]
    fn test_ten(){ assert_eq!(ten(), 10); }
}
