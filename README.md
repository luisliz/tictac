# tictac 
self-hosted ticktick clone made (almost) fully by AI.

I'm not an expert in rust or flutter so please be careful, suggestions are welcome. I'm using this as an excuse to learn rust, flutter & using these AI tools. 

**This is a proof of concept, not a production ready app.**



## Goal 
- To make a proof of concept of using ai tools and at the same time creating a self-hosted ticktick clone. 
- Flutter and Rust were chosen because I wanted to learn them, but also because they get the most bang for the buck in terms of performance and cross-platform support.

## AIs used 
- Aider (w/ GPT-4 api)
- Claude.AI 
- GPT-Engineer

## Prompts 
- I didn't save prompts since I didn't really do anything special I mostly gave the data to claude and then refined based on each response from the respective AI. Commits have a lot of the prompts made by aider
- It's important to hold the AI's hand and go step by step instead of trying to do it all at once. e.g. instead of "Implement all the handlers" I would do "Implement the tasks.dart based on tasks.rs".

## Workflow 
### Day 1
#### Api & Design 
    - I took the full ticktick website and used https://claude.ai in order to take advantage of the large context and had a long conversation about the new design and how to implement it. 
    - With the spec I then passed to **gpt-enginner**. It asked questions those got fed back to claude to get precise answers. I had to spend a lot of time refining the questions and answers so each could understand. 
    - With the fundamentals down I used **aider** to complete the rest. Here I bounced between claude and aider to get the best results.
    
#### UI 
    - I started by using aider in combination with the rust api to implement the UI. But I thought maybe using gpt-engineer together with a spec from claude would be a better option. 
    - I tried to use gpt-enginner but giving the rust api reference with aider seems to work better for me. 

### Day 2 
#### Design 
- I made a script that merges all of the files and passed that to **claude.ai** to see what I was missing and come up with a plan. I told it to divide into sprints by backend/ui and refined. 
- Switched to gpt-3.5 to lower costs and give more context. 

## Conclusions
- Now I just have to spend a long time debugging the code and fixing the errors. I could leverage ai but I think it would be faster to do it myself.

### Costs
- ~ $15 gpt-4 api.
- Day 1: 6 hours | Day 2: 6 hours (and counting)

### Pitfalls 
- Some files it placed them in totally the wrong place. I had to rearrange a lot. 

### Results 
TBD 
