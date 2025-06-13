# Software vulnerabilities - Bayesian theorem

Here I'm leaving my toughts on how to use bayesian theorem to predict: how compromise is a device based on certain evidence.

Okey .. I'll try to write my ideas about how to designed and resolve this problem.

Formula: 
P(H | E) = P(E | H) * P(H) / P(E)

This problem, for a better understanding, should be seen as:
1. A probability of a device to be compromised based on given evidence... P(H|E) ... 
2. P(H | E) is calculated based on the probability of a evidence found in the universe of compromised devices P( E | H ).
3. It multiply by the probability of the hypotesis in the universe of devices.
4. Divided by the probability of seen the evidences in the all universe of cases P (E) ... 

okey now how can I translate this to a data, or programming language. Well I can start with an structure that represent a device id, and in it a vector of [severity, cvss_score, exploitability, update_available] 

1. What is the universe, I'm trying to see if this specific device could be compromise based on evidence .. and actually what is really compromise? I should then split the all universe in compromise devices and not compromise devices? and what if a devices has multiple 
kind of vulnerabilities where some of them are "compromisable"and others not, in what group would I consider this devices.

    1.2 . to get P(H), probabilty of compromises devices I should first define what is compromise, I'm thinking now for example, if I have a device with a vector of vulnerabilities, the devices should be already consider compromise if some of is vulnerabilities are critical, 
    exploitable. And because I have access to the historical data I could already calculate P(H), probability of the hypotesis, or in other words, probability to get a compromise device.

3. First I should define what is evidence: Extra info that comes with the data as could work as evidence. Severity, cvss_score, exploitability, update_available, now my doubt is, how can I summarize this evidence in a way that I can look for this in the all universe 
of devices, and check there for more devices with this characteristics .. 


Now P(E) ... mmm .. on how many devices have the "evidence characteristics"in the all universe of compromised and not compromised ...

## Answer to questions above

1. Universe: Mark each device as compromise based on a new "proxy indicators", that is basically a new column based on other columns ... I'll create a new column "compromise" on each device, it'll be based on Severit, exploitable, update_available.

if (severity = "critical" & explitable != "No exploitablr" & update_availabe = false) { compromise = 1 } else { compromise = 0 }; 

2. Evidence: Evidence is a set of features to characterize each vulnerability presente on a device. 

I still not sure how this would work. If I define [severity, cvss_score, exploitability, update_available] as my evidence, on each structure or device I'll have several vulnerabilities, each of them with their own evidence/ vulnerabilities. Would evidence 
then a different universe of cases to see? for example, thinking on a matrix of compromised and not compromised cases, each of then will have an array of vulnerabilities, each time I want to evaluate a new case I would need to count how many devices share this same "evidence characteristics
to calculate anything new. What I mean is each time I'm trying to figured out the probability of a compromised devices based on tis own characteristics, I should look into the array of vulnerabilities of each device and see if they have the same evidence to calculate for example
P(E | H) or P( E), at the end P(E) is a dynamic value, while P(H) is static. I'd just define one time the universe of compromised and not compromised, and after I'll be looking for the evidence in this two groups.

                                                                
device1: [{
            "severity": "ciritcal",
            "cvss_score": "3-5",
            "exploitability: "noExploit",
            "update_available": "true",
        },
        {    "severity": "ciritcal",
            "cvss_score": "3-5",
            "exploitability: "noExploit",
            "update_available": "true",
        },
        {
            "severity": "ciritcal",
            "cvss_score": "3-5",
            "exploitability: "noExploit",
            "update_available": "true",
        }]
device2: [{
            "severity": "ciritcal",
            "cvss_score": "3-5",
            "exploitability: "noExploit",
            "update_available": "true",
        },
            "severity": "ciritcal",
            "cvss_score": "3-5",
            "exploitability: "noExploit",
            "update_available": "true",
        },
            "severity": "ciritcal",
            "cvss_score": "3-5",
            "exploitability: "noExploit",
            "update_available": "true",
        }]

