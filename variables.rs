// Type of variables or data type rust supports

fn main() {
      // Now define a variable
    let my_var_1 = 10;
    println!("my_var_1 = {}", my_var_1);
    //my_var_1 = 12; // error

    // 2: Introduction of mutablity
    // the value of above variable can't be modified because in Rust, all variables are
    // by default immutable so here is the way if you want to modify the value

    let mut my_var_2 = 20;
    // The keyword 'mut' is to disable that behaviour, now value can be modified.
    println!("my_var_2 = {}", my_var_2);
    my_var_2 = 50;
    println!("updated my_var_2 = {}", my_var_2);

    // 3:  Type Identification
    let my_name = "Amber"; // Implicitly type identification as '&str'
    println!("implicit my_name = {}", my_name);
    let my_name: &str = "Amber"; // explicitly saying this variable is meant for strings only
    println!("explicit my_name = {}", my_name);

    // Rust is intelligent enough to identifying the type of a variable then why do we need
    // explicit type declaration for a variable?
    // Consider a case of a bool variable => let my_bool = "true"
    // As the above variable's value is under "double quote" Rust considers it as String so
    // to overcome from this problem, a explicit cast is required
    let my_bool = "true";
    println!("my_bool = {}", my_bool);
    let my_bool_updated: bool = true;
    println!("my_bool_updated = {}", my_bool_updated);
    let var = false;
    println!("var = {}", var);
}
