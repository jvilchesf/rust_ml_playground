# [Day 1: Probability basics & conditional probability]({"attribution":{"attributableIndex":"0-4"}})  

## Notes

Due to this is a personal space for me to learn, I'll start just writing a definition of what conditional probability is. I'll look for a simple example and try to implement it in rust.

### Definition

Base on what wikipedia says, in probability theory, conditional probability is a measure of the probability of an event ocurring, given that another event
is known that have occurred. This method relies on an event A ocurring with some sort of relationship with another event B. Then would be possible to analyze
the behavior of the event A with respect to the event B.

The comun way to express conditional probability is "The probability of A under the condition B", and it is usually written as P(A|B). Just as a note, it
is not really important, it could be seen or understand as, the fraction of the probability of B that intersect with A. 

P(A|B) = P(A ∩ B) / P(B)

Example:

    - The probability of somehone having a cough is around 5%, but when this person is sick, the probabilities rise to 75%, and it would be epxressed as
      P(cough | sick) = 75%

Now, based on other sources of data. I got a different, not that formal definition that makes sense also. Other way to see conditional probabilities
is like, update the original probability with new information. for example, if we know that the probabilties of rain on any day of the next week is 23%,
the on Monday maybe I'd say that probabilities of Wednesday of rain are 23%, but what happen if now in Wednesday the sky is very cloudly? I could 
use this new information to update my probabilities. 

P(rain | cloudy) = P(rain ∩ cloudy) / P(cloudy)

Okey, I'm getting the idea, but what now if I try with a more numeric example, the classic roll a dice. If a roll a fair dice, let A be the event
that the outcome is and odd number, i.e., A = { 1, 3, 5}. It means we want to know the probabilities of rolling a dice and get a odd number. But as
extra information we know that the outcome will be a number less or equal to 3, B = {1, 2, 3}. What is then the probabilty of get an odd number?

```
P( {1 ,3 , 5} | {1, 2,  3}) = P( {1,3,5} ∩ {1,2,3} ) / P(1,2,3) = P {1,3) / P {1,2,3) = 2 / 3 = 0.666    
