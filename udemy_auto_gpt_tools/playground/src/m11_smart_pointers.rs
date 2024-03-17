#[cfg(test)]
#[allow(dead_code)]
mod test {

    // cargo test tests_box_smart_pointer -- --nocapture
    #[test]
    fn tests_box_smart_pointer() {
        println!("\n---> mod 11 test start\n");

        let _x = Box::new(50);

        #[derive(Debug)]
        struct Node {
            id: u32,
            next: Option<Box<Node>>,
        }

        let nodes = Box::new(Node {
            id: 0,
            next: Some(Box::new(Node { id: 1, next: None })),
        });

        dbg!(nodes);
    }

    use std::rc::{Rc, Weak};
    use std::cell::RefCell;

    // cargo test tests_refs_counter -- --nocapture
    #[test]
    fn tests_refs_counter() {

        // does not work..use Rc...
        // let mut x = 50;
        // let y = &x;
        // x = 70;
        // dbg!(y);


        // case: Rc..allows us to print y... and x keeps new value 70
        //
        let mut x = Rc::new(50);
        let y = Rc::clone(&x);
        x = Rc::new(70);
        dbg!(x);
        dbg!(y);

        // case: RefCell..allows us to change x..with "borrow-fn"
        //
        let x2 = Rc::new(RefCell::new(50));
        let y2 = Rc::clone(&x2);
        *x2.borrow_mut() = 70;
        dbg!(x2);
        dbg!(y2);        


    }



   

    // cargo test tests_weak_ref -- --nocapture
    #[test]
    fn tests_weak_ref() {

        // 1. we could "house_1.clone()" but its better to have an ref of org..


        // #[derive(Debug)]
        // struct House {
        //     address_number: u16,
        //     street: String,
        //     furnitures: Vec<Furniture>
        // }
    
        // #[derive(Debug)]
        // struct Furniture {
        //     id: String,
        //     desc: String,
        //     house: House
        // } 

        //
        // let house_1 = House{
        //     address_number : 1,
        //     street: "street1".to_string(),
        //     furnitures: vec!()
        // };

        // let table_1 = Furniture{
        //     id: "table1".to_string(),
        //     desc: "desc1".to_string(),
        //     house: house_1
        // };


        // let desk_1 = Furniture{
        //     id: "desk2".to_string(),
        //     desc: "desc2".to_string(),
        //     house: house_1
        // };  


        // 2. we could try Rc, RefCell...but it will create a CIRCULAR-REF..solution 3 fixes that with Weak-type..
        //
        // #[derive(Debug, Clone)]
        // struct House {
        //     address_number: u16,
        //     street: String,
        //     furnitures: RefCell<Vec<Rc<Furniture>>>
        // }
    
        // #[derive(Debug, Clone)]
        // struct Furniture {
        //     id: String,
        //     desc: String,
        //     house: Rc<House>
        // } 

        // let mut house_1: Rc<House> = Rc::new(House{
        //     address_number : 1,
        //     street: "street1".to_string(),
        //     furnitures: RefCell::new(vec!())
        // });

        // let table_1: Rc<Furniture> = Rc::new(Furniture{
        //     id: "table1".to_string(),
        //     desc: "desc1".to_string(),
        //     house: Rc::clone(&house_1)
        // });


        // let desk_1: Rc<Furniture> = Rc::new(Furniture{
        //     id: "desk2".to_string(),
        //     desc: "desc2".to_string(),
        //     house: Rc::clone(&house_1)
        // });   
        // house_1.furnitures.borrow_mut().push(Rc::clone(&table_1));            
        // house_1.furnitures.borrow_mut().push(Rc::clone(&desk_1));        
        // dbg!(house_1);    


        // 3. add WEAK type..fix issues in 2.
        //
        #[derive(Debug, Clone)]
        struct House {
            address_number: u16,
            street: String,
            furnitures: RefCell<Vec<Rc<Furniture>>>
        }
    
        #[derive(Debug, Clone)]
        struct Furniture {
            id: String,
            desc: String,
            house: Weak<House>
        } 

        let house_1: Rc<House> = Rc::new(House{
            address_number : 1,
            street: "street1".to_string(),
            furnitures: RefCell::new(vec!())
        });

        let table_1: Rc<Furniture> = Rc::new(Furniture{
            id: "table1".to_string(),
            desc: "desc1".to_string(),
            house: Rc::downgrade(&house_1)
        });


        let desk_1: Rc<Furniture> = Rc::new(Furniture{
            id: "desk2".to_string(),
            desc: "desc2".to_string(),
            house: Rc::downgrade(&house_1)
        });   
        house_1.furnitures.borrow_mut().push(Rc::clone(&table_1));            
        house_1.furnitures.borrow_mut().push(Rc::clone(&desk_1));        
        dbg!(house_1);         


    }    


}
