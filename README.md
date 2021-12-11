# Banana Fuzzer
---
## what
Platform agnostic fuzzer for state-aware targets, focused on race conditions, written in Rust with bananas, OOP and FP. Highly modulable, loop based approach with automatic poc recording, and replay option, thus code coverage apply. 

## docs
I think, it will be one of not many projects where you actually more appreciate code than comments of the code, thanks to my chinglish :) But in general i did not make any official documentation for it yet, maybe in future. Easier to read code, it is by no means big code base. start from crates in this order : ```api -> core -> fuzzer```, every source to check check, you will understand.

## how 
In fact it is PITA to build fuzzing logic as some declaration needed, no internal format to interpret, but actual rust code. Automation script for this wanted, i have one i am using but it is older version and i am too lazy to update it. It would be much appreciated if you can make and push your version of it - call for action ;)
- but in general you have toy example linux socket, in branch "toy"
- there you can get idea
- basically anything in fuzzer crate, except some common code in main.rs

Once i will polish my version, or better to say - write one from scratch, i will put it here too, but i dont promise that will be anytime soon..

## why
- i am putting it on github as tribute i should pay towards open source and other open sourced research i learned from, but it is not intended to be fully fledged fuzzer clone + use; rather you can tinker it with it if you want and contribution to its developement overtime will be appreciated. I do plan to maintain it.


- name ~ once you eat 20+ bananas a day, except other goodies, you will think everythink is banana


- re:syzkaller ~ this work does not aim to be better or replace syzkaller, it simple adds another angle :
    + black box, little or no code cov feedback
    + researcher knowledge as main feedback
    + race conditions as main priority ( every object fuzzer has its own thread )
    + plugins for everything
    + you can stack bananafuzzer to syzkaller ( trough plugins )
      * using syzkaller manager
      * using syzkaller corpora
      * forward banana fuzzer corpora to syzkaller
    + oh yeah, and banana fuzzer is slower by default, especially when more plugins involved, but tbh having edge at performance was never the goal here  
      
## TODO : 

> in following weeks i will add plugins and tools for code coverage, meanwhile, if you are interested, you can check and think, how would you do that trough plugin option of banana fuzzer.

> plugin documentation, that one i will make for sure, as a blog post to cover general idea behind

## Call for action ~ feature requests
  - i have obsolete version of plugin for syzkaller, esentially stacked banana fuzzer to syzkaller manager as replacement for syzkaller executor, this one i dont plan to put on github, as obsolete and not too much clean - hacky version, and i have other idea i want to put my time into. Though for anybody who want to look at that version of my plugin, you can ping me on twitter i can share with you privately.
  
 
  - that beeing said, if you do pluging for stacking banana fuzzer to syzkaller manager for current version feel free to push it here
  - syzkaller template -> bana fuzzer fuzzing code generation
    - best if python script
    - should be 1:1 template of syzkaller descriptions
  - syzkaller corpora adapter for banana fuzzer
    - how to do it will be more clear once i share my code coverage plugins + tools
  - server GUI, simple terminal one will be nice 
    + ratio of sucessfull syscalls/packets per syscall/packet type
    + performance
    + code cov info
    + crash monitoring
  - server GUI, trough browser
    + all what in terminal
    + heatmap overall ( over syscalls/packet handlers )
    + heatmap per syscall ( likely with option to click on syscall and will display colored graph w/o disasembly )
    
## misc

| neg  | fix |
|:-:|:-:|
| PITA to generate  | could be automated  |
| slower than other fuzzers | use other fuzzers corpora and apply your knowledge by banana  |
| no server for code coverage | adapt to existing servers via plugins |
| do not finding any bugs | apply better logic to fuzzing, analyse feedback, study target |
| missing functionality | write plugin |
| i need to inject to process | build fuzzer as a library |


> note : check configs config.toml ( general fuzzer behaviour ), modules.toml ( specifics per plugin settings )


## toy example

- presented config.toml is not designed for code cov ( lot of object generated, bit queue allowed, lot calls per object allowed )
- "fuzzing" is restricted just for testing - templates (sock_addr.rs, socket.rs) just few flags allowed 
- object logic (fuzzer/src/states/socket/state.rs) nicely demonstrates some edge cases : 
  - linux kernel is fd based, and dup is essential part, in order to properly record for poc we need special logic ( basically it applies for any target with cored dup functionality ) : SocketState->dup function
  - accept seems only callable from server main thread, and it creates new object, need to manualy add to fuzzing queue : SocketState->fuzz_one function
    + handling two cases : race condition able - need to put multiple objects duped fd to queue, poc reproducibility : need to put object with original fd to enable UAF scenarios
    + normaly, accept should be in constructor ( it is but in this case does not help as always fails as server thread needed to be caller ), so any subsequent object created should be responsible for conruction in own thread
  - via calltable we inject, quite restrictive - but just for demonstration, knowledge to the fuzzing ( bind -> listen -> accept | socket -> connect )
  
> there we can see one major pro and con


| pro | con | good practice |
|:-:|:-:|:-:|
| knowledge based, unique per researcher, potentialy different bugs that with code cov only | can backfire, for socket there are many flags, if you want to do knowledge based good fuzzer you need to cover lot of corner cases - limiting full potential fuzzer to knowledge and assumptions of your own  | for such complex target : do code cov fuzzing via more general approach, take corpora and there you can inject logic on top of it |
| does not necesary need code cov feedback to do good fuzzing | when adding code cov, in a way that  banana fuzzer will be creating you corpora, you need to disable most of calltable logic to let it more freedom | generate two calltables and runtime switch trough config switch - opla need to implement that :) |
| you can easily implement logic over corpora, fuzz object from selected socket state, or in specific state disable part of fuzzing functionality as that will not apply anymore, .. | will slow down fuzzing - every creation of object in poc you need to backpropagate to runtime banana fuzzer observers | do mixture : corpora fuzzing, and pure knowledge fuzing, where you update knowledge based on corpora and fuzzing feedback / goals |
