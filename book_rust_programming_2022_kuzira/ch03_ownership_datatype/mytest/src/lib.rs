pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug,PartialEq)]
struct GItem {
    name: String,
    price: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_not_works() {
        let result = add(2, 30);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_array_test() {
        let a1 = [1,2,3];
        let a2 = [1,2,3];
        assert_eq!(a1, a2);

        let a3 = ["사과".to_string(), "바나나".to_string()];
        let a4 = [String::from("사과"), String::from("바나나")];
        assert_eq!(a3, a4);
    }

    #[test]
    fn it_vec_test() {
        let v1: Vec<&str> = vec!["apple", "banana"];
        let mut v2: Vec<&str> = Vec::new();
        v2.push("apple");
        v2.push("banana");
        assert_eq!(v1, v2);
    }

    #[test]
    fn it_struct_test() {
        let apple1 = GItem {
            name: String::from("사과"),
            price: 2400,
        };

        let mut apple2 = GItem {
            name: "사과".to_string(),
            price: 0,
        };
        apple2.price = 2400;

        assert_eq!(apple1.name, apple2.name);
        assert_eq!(apple1.price, apple2.price);

        assert_eq!(apple1, apple2);
    }

}
