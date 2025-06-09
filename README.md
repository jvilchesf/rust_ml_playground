# 🦀 rust_ml_from_scratch

**A 6‑week, 3‑hours‑a‑day journey to reinforce statistics and ML fundamentals by implementing them from scratch in Rust.**

---

## 🎯 Project Goals

- ^[Reinforce key concepts in **Statistics & Probability**, **Machine Learning**, **Deep Learning**, **NLP**, **ML Ops**, **SQL**, and **Business Case Thinking**.]({"attribution":{"attributableIndex":"0-1"}})
- ^[Implement each concept in **Rust** (with occasional Python prototypes).]({"attribution":{"attributableIndex":"0-2"}})
- ^[Build a public, well-documented GitHub repo showcasing your learning and code.]({"attribution":{"attributableIndex":"0-3"}})

---

## 🗓️ 6‑Week Study Plan

### Week 1–2: Statistics & Probability
- ^[Day 1: Probability basics & conditional probability]({"attribution":{"attributableIndex":"0-4"}})  
- ^[Day 2: Bayes’ theorem applications]({"attribution":{"attributableIndex":"0-5"}})  
- ^[Day 3: Distributions – Normal, Binomial, Poisson]({"attribution":{"attributableIndex":"0-6"}})  
- ^[Day 4: CLT & LLN]({"attribution":{"attributableIndex":"0-7"}})  
- ^[Day 5: Z‑tests & t‑tests]({"attribution":{"attributableIndex":"0-8"}})  
- ^[Day 6: p‑values & confidence intervals]({"attribution":{"attributableIndex":"0-9"}})  
- ^[Day 7: Chi‑square & ANOVA]({"attribution":{"attributableIndex":"0-10"}})  

### Week 3: Core Machine Learning
- ^[Day 1: Linear Regression (theory + Rust)]({"attribution":{"attributableIndex":"0-11"}})  
- ^[Day 2: Logistic Regression]({"attribution":{"attributableIndex":"0-12"}})  
- ^[Day 3: Decision Trees (Gini, Entropy)]({"attribution":{"attributableIndex":"0-13"}})  
- ^[Day 4: Random Forest & Boosting]({"attribution":{"attributableIndex":"0-14"}})  
- ^[Day 5: k‑NN & k‑Means]({"attribution":{"attributableIndex":"0-15"}})  
- ^[Day 6: SVM intuition]({"attribution":{"attributableIndex":"0-16"}})  
- ^[Day 7: Recommender basics]({"attribution":{"attributableIndex":"0-17"}})  

### Week 4: Evaluation & Preprocessing
- ^[Day 1: Accuracy, Precision, Recall, F1]({"attribution":{"attributableIndex":"0-18"}})  
- ^[Day 2: ROC‑AUC & PR‑AUC]({"attribution":{"attributableIndex":"0-19"}})  
- ^[Day 3: Missing data & scaling]({"attribution":{"attributableIndex":"0-20"}})  
- ^[Day 4: Encoding categorical variables]({"attribution":{"attributableIndex":"0-21"}})  
- ^[Day 5: Outlier detection]({"attribution":{"attributableIndex":"0-22"}})  
- ^[Day 6: Feature selection]({"attribution":{"attributableIndex":"0-23"}})  
- ^[Day 7: Pipeline design]({"attribution":{"attributableIndex":"0-24"}})  

### Week 5: Deep Learning & NLP
- ^[Day 1: Neural networks + backprop]({"attribution":{"attributableIndex":"0-25"}})  
- ^[Day 2: CNN fundamentals]({"attribution":{"attributableIndex":"0-26"}})  
- ^[Day 3: Optimizers (SGD, Adam)]({"attribution":{"attributableIndex":"0-27"}})  
- ^[Day 4: Regularization & dropout]({"attribution":{"attributableIndex":"0-28"}})  
- ^[Day 5: Tokenizers & embeddings]({"attribution":{"attributableIndex":"0-29"}})  
- ^[Day 6: Transformer & attention mechanism]({"attribution":{"attributableIndex":"0-30"}})  
- ^[Day 7: BERT & GPT concepts (pretraining, LoRA/PEFT)]({"attribution":{"attributableIndex":"0-31"}})  

### Week 6: SQL, MLOps & Business Thinking
- ^[Day 1: SQL joins, window functions]({"attribution":{"attributableIndex":"0-32"}})  
- ^[Day 2: ML system design & versioning]({"attribution":{"attributableIndex":"0-33"}})  
- ^[Day 3: Model drift monitoring]({"attribution":{"attributableIndex":"0-34"}})  
- ^[Day 4: Dockerize a model]({"attribution":{"attributableIndex":"0-35"}})  
- ^[Day 5: Cloud deployment basics]({"attribution":{"attributableIndex":"0-36"}})  
- ^[Day 6: A/B testing & statistical power]({"attribution":{"attributableIndex":"0-37"}})  
- ^[Day 7: End‑to‑end case study]({"attribution":{"attributableIndex":"0-38"}})  

---

## 🔧 Daily Routine (3 hrs/day)

1. ^[**1 hr** – Theory & note-taking]({"attribution":{"attributableIndex":"0-39"}})  
2. ^[**1 hr** – Implement code in Rust (optionally Python first)]({"attribution":{"attributableIndex":"0-40"}})  
3. ^[**1 hr** – Write daily `README.md` entry, push commits]({"attribution":{"attributableIndex":"0-41"}})  

^[This structure reflects best practices from popular study-plan repos]({"attribution":{"attributableIndex":"0-42"}})  [oai_citation:0‡github.com](https://github.com/patrickloeber/ml-study-plan/blob/master/README.md?utm_source=chatgpt.com) [oai_citation:1‡github.com](https://github.com/nogibjj/rust-mlops-template?utm_source=chatgpt.com) [oai_citation:2‡github.com](https://github.com/huggingface/candle?utm_source=chatgpt.com) [oai_citation:3‡codezup.com](https://codezup.com/building-machine-learning-models-with-rust-ml-library/?utm_source=chatgpt.com).

---

## 🪻 Getting Started

```bash
# Initialize the project
^[cargo init rust_ml_from_scratch]({"attribution":{"attributableIndex":"2498-0"}})
cd rust_ml_from_scratch

# Optional: set up Python virtualenv for prototyping
^[python3 -m venv .venv]({"attribution":{"attributableIndex":"2498-1"}})
^[source .venv/bin/activate]({"attribution":{"attributableIndex":"2498-2"}})
^[pip install jupyter numpy pandas matplotlib]({"attribution":{"attributableIndex":"2498-3"}}
