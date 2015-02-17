#![allow(unstable)]

#[allow(unused)]
fn main() {
    println!("Hello, world!");
}

struct PCell {
    val: i32,
    next: Box<&PList>
}

struct PList {
    cell: Option<PCell>
}

impl PList {
    fn new() -> PList {
        PList { cell: None } // or cell can be (val, PList) tuple
    }

    fn len(&self) -> u32 {
        0
    }

    fn append(&self, i:i32) -> PList {
        PList { cell: Some(
            PCell { val: i, next: Box::new( &*self ) }
        )}
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::PList;

    #[test]
    fn can_instantiate_empty_list(){
        PList::new();
    }

    #[test]
    fn empty_list_length_is_zero(){
        assert_eq!(0, PList::new().len());
    }

    #[test]
    fn can_append_to_a_list(){
        let p = PList::new().append(1);
    }
    
}
