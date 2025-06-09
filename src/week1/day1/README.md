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

```
The probability of somehone having a cough is around 5%, but when this person is sick, the probabilities rise to 75%, and it would be epxressed as
P(cough | sick) = 75%
```
Now, based on other sources of data. I got a different, not that formal definition that makes sense also. Other way to see conditional probabilities
is like, update the original probability with new information. for example, if we know that the probabilties of rain on any day of the next week is 23%,
the on Monday maybe I'd say that probabilities of Wednesday of rain are 23%, but what happen if now in Wednesday the sky is very cloudly? I could 
use this new information to update my probabilities. 

P(rain | cloudy) = P(rain ∩ cloudy) / P(cloudy)

Okey, I'm getting the idea, but what now if I try with a more numeric example, the classic roll a dice. If a roll a fair dice, let A be the event
that the outcome is and odd number, i.e., A = { 1, 3, 5}. It means we want to know the probabilities of rolling a dice and get a odd number. But as
extra information we know that the outcome will be a number less or equal to 3, B = {1, 2, 3}. What is then the probabilty of get an odd number?

```
P({1, 3, 5} | {1, 2, 3}) = P(A ∩ B) / P(B)
P({1, 3, 5} ∩ {1, 2, 3}) / P(1, 2, 3)
P {1, 3) / P {1, 2, 3) 
2 / 3 = 0.666 # 2 is the number of options that I have, and 3 is the universe of possible cases
```
Here is the thing, because we kno that B has ocurred, every outcome that is outside B should be discarded.
Thus, our sample space is reduced to the set B. Now the only way that A can happen is when the outcome 
belongs to the set A ∩ B.

![Conditional probability graphically](https://www.probabilitycourse.com/images/chapter1/conditional_b.png)


Example:

I roll a fair die twice and obtain two numbers X1= result of the first roll and X2=result of the second roll. Given that I know X1+X2 = 7, what is the 
probability that X1=4 or X2=4?

```
P(x1 = 4 or x2 = 4 | x1 + x2 = 7) = P( A  ∩  B) / P(B)
# First calculate the probabilty of just A happening, I've considered all cases where 4 happen in eithere dice a or b.
P(A) =(4, 1), (4, 2), (4, 3), (4, 4), (4, 5), (4, 6), (1, 4), (2, 4), (3, 4), (5, 4), (6, 4)
# Calculate the probabily of the B cases happening, when sum of both dices rolled is 7.
P(B) = (1, 6), (2, 5), (3, 4), (4, 3), (5, 2), (6, 1)
# Intersect both probabities
P(A ∩ B) = (3, 4) , (4,3) = 2 / 36
# count how many cases I can get 7 rolling both dices
P(B) = 6 / 36 

P(A ∩ B) / P(B) = (2 / 36) / (6 / 36)  
(1 / 18) / (1 / 6) = 6 / 18 = 1 / 3 = 0.3333%
```


