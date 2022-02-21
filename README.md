# Bananized Fuzzy Loop; by bananafzz + LibAFL
---
## what
Aims to be SOTA fuzzer for state aware fuzzing : Kernels, Virtualization layers, Network protocols.. targeting deep bugs via race conditions.

## why
As bananafzz shines when it comes to blackbox settings + race conditions, it lacks server and mechanism for code coverage automation. And as (Lib)AFL is SOTA in this are, I aim to (mis)use LibAFL as server for this very purpose, to acquire deep corpus for target, and balance bananafzz itself. That way I can use balanced fuzzer ( reach, targeted code block / targeted areas of code, in a proportional way ) with "deep corpus" ( containing hard to get code paths, and filtered uninterested low fruits ) for real fuzzing of RACE CONDITIONS afterwards. 

## wip
for now established LibAFL + bananafzz cooperation, need to add bijon ( Bananized IJON ) to the game, setup more comprehensive toy example of usage ( Super Mario Bros 2 with multiple controls input per action )


## eta when
I will make blog post when I will polish it, and have version matured enough to showcase why eating LibAFL cpu time over bananafzz is worhwhile to do.

## ps
go get some bananas, they are good :)

