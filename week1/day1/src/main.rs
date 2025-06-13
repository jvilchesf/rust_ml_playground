/*
Example:

I roll a fair die twice and obtain two numbers X1= result of the first roll and X2=result of the second roll. Given that I know X1+X2 = 7, what is the 
probability that X1=4 or X2=4?

*/
mod operations;

fn main() {
    /*
    * Function to develope an easy conditional prbabilty example in rust.
    * My idea here is practice rust and stats at the same time
    * This first example is about roll two dices, and calculate the probability of getting and x
    * value based, knowing the sum of the result of rolling the two dices
    */
    
    // 1. Declare two arrays that will represent both dices
    let dice1 = vec![1, 2, 3, 4, 5, 6];
    let dice2 = vec![1, 2, 3, 4, 5, 6];
    
    // 2. Declare what I want to calculate P(A), and what I know P(B)
    // In this example I want to calculate what are the changes to get a 4 on each of the dices
    let x: u32 = 4;
    let roll_dice_result: u32 = 7;

    // Declare hash table to save searched combinations
    let mut list_pa: Vec<(u32, u32)> = Vec::new();
    let mut list_pb: Vec<(u32, u32)> = Vec::new();
    let mut list_intersection: Vec<((u32, u32), (u32, u32))> = Vec::new();

    // Call function to get a vector with tuples for all possible values
    operations::create_pa(x, &dice1, &dice2, &mut list_pa);

    println!("Lista pa: ");
    for (x, y) in &list_pa{
        println!(" ({} -> {})", x, y);
    }

    // Call function to get a vector with tuples of all values due the condition x + y = 7
    operations::create_pb(roll_dice_result, &dice1, &dice2, &mut list_pb);

    println!("Lista pb: ");
    for (x, y) in &list_pb{
        println!(" ({} -> {})", x, y);
    }
   
    // 3. Look for the intersection of both group  P( A  ∩  B)
    operations::intersection_two_vectors(&list_pa, &list_pb, &mut list_intersection);

    println!("Intersection two vectors is P( A  ∩  B)");
    for intersection in &list_intersection{
        println!("{:?}", intersection);
    }

    println!("Length of intersection vector is: {}", list_intersection.len());

    // 4. Calculate the intersection / universe
    let conditional_probability : f64 = list_intersection.len() as f64 / list_pb.len() as f64;

    println!("The conditional probability {} / {}", list_intersection.len(), list_pb.len());
    
    println!("The conditional probability {}", conditional_probability);

}
