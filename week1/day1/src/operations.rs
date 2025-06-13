pub fn create_pa(x: u32,
            dice1: &Vec<u32>,
            dice2: &Vec<u32>, 
            list_pa: &mut Vec<(u32, u32)>,
    ) {
    /*
    Calculate PA separated.
    PA is the probability that is been searched.
    For example: What is the probability of any of this two dices to be a number 4?
    */
    // Use for to get all the possible cases when "4" shows up
    
    // Save the possible cases in a list tuples
    for i in dice1.iter(){
       for j in dice2.iter(){
           if (*i == x) || (*j == x){
                list_pa.push((*i, *j));
            }
        }
    }   
}

pub fn create_pb(roll_dice_result: u32,
                dice_1: &Vec<u32>,
                dice_2: &Vec<u32>,
                list_pb: &mut Vec<(u32, u32)>
                ){
    /*
    pb is the universe of cases known based on the condition or extra information given.
    In this case we know that the sum of x1 + x2 = 7
    Seems then that the correct way is to find all possible cases when x1 and x2 = 7 in a iteration
    */

    // start iteration over first and second dice 
    for i in dice_1{
        for j in dice_2{
        // look for cases when the sum of x + y = 7
            if (*i + *j) == roll_dice_result{
            // If that is the case push into the list_pb vector
                list_pb.push((*i, *j))
            }
        }
    }
}

pub fn intersection_two_vectors(list_pa: &Vec<(u32, u32)>,
                                list_pb: &Vec<(u32, u32)>,
                                list_intersection: &mut Vec<((u32, u32), (u32, u32))>
    ){
    /*
    Iterate over two lists looking for the tuples that are the same
    */

    for i in list_pa{
        for j in list_pb{
            if *i == *j{
                list_intersection.push(((*i), (*j)));
                }
            }
        }

    }


