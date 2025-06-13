# [Day 2: Bayes’ theorem applications]

This text is about understanding basics of Bayes theorem, and get some examples to apply them in rust.

## Definition

Gives a mathematical rule for inverting "conditional probabilies", allowing one to find the probability of a cause
give its effect. 

Okey I've been watching the video for bayes theorem in 3Blue1Bron https://www.youtube.com/watch?v=HZGCoVF3YvM, and I want to write down some of the main 
ideas that I got from there.

The main differece between conditional probabilies and bayes therome is that, in conditional probabilities you are really focus on the probability of one event
due to some other event already ocurred, in the card example, you could try to calculate the probablity of getting a "K", knowing that the card that I just took
is a face car. Then I'll really need to focus on the probability of the ? event base on the known event = probability of both together / probabilty of the known event alone.

I mean has I just wrote, a lot is about events and probability based on events. In the case of Bayes theorem the deal is different. It is really focus on update
the probablity of a hypotesis given certain evidence, or P(H|E). The formula can be a bit intimidating a confusing...

It's about updating beliefs based on evidence, not just calculating probabilities given known events.

P(H | E) = P(E | H) * P(H) / P(E)
P(H | E) = it is describe as a "Prior probability"

P(H | E): Probability of the hypotesis happening based on certain evidence

P(E | H): Probability of the evidence happening in the all universe of Hypotetical cases (it'll make more sense in the example)

P( H ) : Probability of the hypotesis happening.

P( E ) : Probability of the evidence happening.

Okey then ... If I see this alone doesn't make much more sense, but after the example with librarian and farmer example it could be more clear.

Suposse that somebody ask you, Steve is a very shy and withdrawn, invariably helpful but with very little interest in people or in the world of reality. A meek
and tidy soul, he has a need for order and structure, and a passion for detail.
The question is: Steve is a librarian or is a farmer? 

P(H) = 1/21 (prior probability Steve is a librarian, given the 1:20 ratio)
P(E|H) = likelihood that a librarian has Steve's characteristics
P(E) = total probability of having Steve's characteristics (across all professions)

Based on the intuition most people would answer that steve is a librarian, but the problem is when you star looking for volumne or magnitude of cases ...
We could say or have as a data that there are 1:20 librarias to farmers relatiion, means that for 1 labrarian we have 20 farmers.


                                        L F F F F F F F F F F F F F F F F F F F F 
                                        L F F F F F F F F F F F F F F F F F F F F 
                                        L F F F F F F F F F F F F F F F F F F F F 
                                        L F F F F F F F F F F F F F F F F F F F F 
                                        L F F F F F F F F F F F F F F F F F F F F 
                                        L F F F F F F F F F F F F F F F F F F F F 
                                        L F F F F F F F F F F F F F F F F F F F F 
                                        L F F F F F F F F F F F F F F F F F F F F 
                                        L F F F F F F F F F F F F F F F F F F F F 
                                        L F F F F F F F F F F F F F F F F F F F F 
                                        L F F F F F F F F F F F F F F F F F F F F 

Okey, I made this square to represent the universe, and magnitude of difference between farmers and librarians, and to calculate to represent the bayesian theoreme
formula from it.

If I know would answer the Steve question, I'd try to understand each of the formulas part.

first, the evidence is that Steve is a person with person with a meek and tidy soul, he has a need for order and structure and passion for a detail.

First, what is P(E | H), what are the chances that a person with the "Evidence characteristics" are a librarian? Like in the video was shown, it is a value already
known, but it represent a fragment of the hypotesis universe, the universe in this case are all librarians, and it is because I've chosen to think that steven is more
probable to be a librarian.

P( H ) : It is the probability of my hypotesis, I mean that steve is a librarian over the all universe 1 / 20

P ( E ) : Probability of the evidence happening, it would include farmers and librarians as well, it would be neccessary to know how many librarians + farmers have
the "evidence characteristics"


Key Insight I'm Missing
The power of Bayes' theorem becomes clear when you consider P(E|H) vs P(E|¬H):

P(E|H): How likely are Steve's traits among librarians? (Maybe 40%)
P(E|¬H): How likely are Steve's traits among farmers? (Maybe 10%)

Even if librarians are more likely to have these traits, the base rate (1:20 ratio) might still make Steve more likely to be a farmer!
