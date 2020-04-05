struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {//we have to declare T just after impl 
                    //so we can use it to specify that we’re 
                    //implementing methods on the type
    fn x(&self) -> &T { //we’ve defined a method named x 
        //on Point<T> that returns a reference to the data 
        //in the field x.
        &self.x
    }
}

/*
We could, for example, implement methods only on Point<f32> 
instances rather than on Point<T> instances with any generic type
*/

impl Point<f32> { //will have a method named distance_from_origin 
                //and other instances of Point<T> where T is not 
                //of type f32 will not have this method defined.
    fn distance_from_origin(&self) -> f32 { //The method measures 
        //how far our point is from the point at coordinates (0.0, 0.0) 
        //and uses mathematical operations that are available only 
        //for floating point types.
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

/*
Generic type parameters in a struct definition aren’t always 
the same as those you use in that struct’s method signatures.
*/
#[derive(Debug)]
struct Point2<T, U> { 
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> { //the generic parameters T and U are declared after impl, because they go with the struct definition.
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> { // The generic parameters V and W are declared after fn mixup, 
                                                                //because they’re only relevant to the method.
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    println!("Generic in method");
    println!("");

    let p1 = Point2 { x: 5, y: 10.4 }; //we’ve defined a Point that 
                    //has an i32 for x (with value 5) and an f64 
                    //for y (with value 10.4). 
    println!("p1 : {:?}", p1);
    let p2 = Point2 { x: "Hello", y: 'c'}; //Point struct that has a string 
                    //slice for x (with value "Hello") and a char for y (with value c)
    println!("p2 : {:?}", p2);

    let p3 = p1.mixup(p2); //Calling mixup on p1 with the argument p2 gives us p3, 
                            //which will have an i32 for x, because x came from p1, and
                            // a char for y, because y came from p2

    println!("p3 : {:?}", p3);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}




