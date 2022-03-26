# Bananized Fuzzy Loop; by bananafzz + LibAFL
---
## what
Race condition oriented fuzzer, based on [loop per thread](https://github.com/rezer0dai/bananafzz/blob/smb2/core/src/banana/looper.rs#L38-L76) instead of [pre-generation](https://github.com/google/syzkaller/). Ability to fully [serialize](https://github.com/rezer0dai/bananafzz/blob/smb2/core/src/generator/serialize.rs) and reproduce input/program, extends with LibAFL (aka [LibBFL](https://github.com/rezer0dai/LibAFL/blob/smb2/libafl/src/mutators/bananizer.rs)) for [code coverage](https://github.com/rezer0dai/bananafzz/blob/smb2/modules/bfl/src/bfl.rs) (but LibBFL not targeting races, its very purpose is to build solid base corpus instead). Modulable for hacking via [plugins](https://github.com/rezer0dai/bananafzz/tree/smb2/modules), injecting [knowledge](https://github.com/rezer0dai/bananafzz/blob/smb2/core/src/state/state.rs#L237-L251) and [logic](https://github.com/rezer0dai/bananafzz/blob/smb2/modules/smb/src/smb.rs) via [fuzzing levels](https://github.com/rezer0dai/bananafzz/blob/smb2/fuzzer/src/states/coins/state.rs#L74-L81) + plugins + [argument](https://github.com/rezer0dai/bananafzz/blob/toy/fuzzer/src/args/epoll/pollfd.rs) [generation](https://github.com/rezer0dai/bananafzz/blob/smb2/fuzzer/src/args/movem.rs), and related custom [configs](https://github.com/rezer0dai/bananafzz/tree/smb2/tools/configs). Therefore available to operate in GrayBox and BlackBox settings too, with option to add [custom](https://github.com/rezer0dai/bananafzz/blob/smb2/modules/bijon/src/lib.rs) [feedback](https://github.com/rezer0dai/LibAFL/blob/smb2/libafl/src/observers/bijon.rs#L197-L220) for BlackBox approaches.

## why
As bananafzz shines when it comes to blackbox settings + race conditions, it lacks server and mechanism for code coverage automation. And as (Lib)[AFL](https://lcamtuf.coredump.cx/afl/) is SOTA in this area, I aim to (mis)use [LibAFL](https://github.com/AFLplusplus/LibAFL) as server for this very purpose, to acquire solid corpus for target, and balance bananafzz itself. That way I can use balanced fuzzer ( reach, targeted code block / targeted areas of code, in a proportional way ) with "deep corpus" ( containing hard to get code paths, and filtered uninterested low fruits ) for real fuzzing of RACE CONDITIONS afterwards. 
However [mutation logic of LibAFL](https://github.com/rezer0dai/LibAFL/blob/smb2/libafl/src/mutators/bsched.rs#L24-L60) need to be reconsidered per fuzzing target, what is efficient to do. Likely minor byte mutations are OKish, but most important will be [insert](https://github.com/rezer0dai/LibAFL/blob/smb2/libafl/src/mutators/bfl.rs) + [crossover](https://github.com/rezer0dai/bananafzz/blob/smb2/modules/bfl/src/crossover.rs) mechanics as argument generation bananafzz should handle more or less by itself.

## LibAFL feature patches

- BFL
  + Allowing LibAFL to mutate arguments of bananafzz ( say syscall arguments ) 
    * bananafzz support by [dump + load](https://github.com/rezer0dai/bananafzz/blob/smb2/core/src/generator/serialize.rs#L38-L68)
    * bananafzz [argument selective](https://github.com/rezer0dai/bananafzz/blob/smb2/api/src/leafs/bfl_leaf.rs) generation by [example](https://github.com/rezer0dai/bananafzz/blob/smb2/fuzzer/src/args/movem.rs#L60-L79) 
    * LibAFL [bananizer](https://github.com/rezer0dai/LibAFL/blob/smb2/libafl/src/mutators/bananizer.rs)
  + [Progressive generation](https://github.com/rezer0dai/LibAFL/blob/smb2/libafl/src/mutators/bfl.rs) of input/poc for LibAFL
  + [Focused](https://github.com/rezer0dai/LibAFL/blob/smb2/libafl/src/mutators/banana.rs#L160-L223) LibAFL per (sys)call ( one unit of input/poc )
 
  + NOTE : fuzzing with BFL need to use [syncer plugin](https://github.com/rezer0dai/bananafzz/blob/smb2/modules/syncer/src/lib.rs) to make it kvazi single threaded, and therefore reproducible, aka not fuzzing for bugs, but for making quality corpus only instead
  + PS2 : no best idea to using too much of knowledge when fuzzing with LibAFL, as it can shoot you in your foot, and hinder path to novel feedback coverage. As your logic is your box, and LibAFL trying to get you of the box :) As an example using plugins like [smb](https://github.com/rezer0dai/bananafzz/blob/smb2/modules/smb/src/lib.rs) with LibAFL is no good idea - but using it without (w/ or w/o LibAFL generated corpus) may be good idea (in practice this one concrete pluging does not work well for SMB2 as we dont have good logic to fuzz SMB2 without LibAFL :). Same applies for intercorporating lot of levels into [the state](https://github.com/rezer0dai/bananafzz/blob/smb2/core/src/state/state.rs#L237-L251) may not be the best, but can be countered by creating AFL-alike copy  of same state but without levels (just ctor and then the rest) and fuzz them together with LibAF
- BIJON
  + LibAFL [bijon observer](https://github.com/rezer0dai/LibAFL/blob/smb2/libafl/src/observers/bijon.rs) exposed to bananafzz
    * TODO : need to kick out lazy_static, and change it probably to OwnedSlice, and update exposing mechanism
  + bananafzz [bijon plugin](https://github.com/rezer0dai/bananafzz/blob/smb2/modules/bijon/src/lib.rs) adding custom feedback to BlackBox fuzzing
    * example of custom feedback is sucessfull sequence of syscall leading to significant state - like creating special file / reached special kernel object state - by syscall feedback / network sequence packet progress ( handshake, auth, .. )
- Rotation AFL Buffer
  + countering *E. Minimization* section of https://www.s3.eurecom.fr/docs/fuzzing22_fioraldi_report.pdf
    * idea here is to instead of keeping fixed minimal corpora, minimal set of inputs trigering same feedback coverage, you will exchange, rotate, entries for the newest one which cover part of corpora which was fuzzer extensively. Aka seems older input, covering part of that coverage, seems does not brings anything new at this point, so changing it for the newest one covering particular part of same corpora should not bring any harm - except breaking "minimal" word from equation. To note, it means that previously edges, when talking about code coverage, A B C D were covered by input1, but now edges A and D are covered by input2, edge B by input 1, and edge D by input 3, quite plausible scenario. Note that expansion of minimal is linear to the unique edges count, which is not too bad.
  + implementation is based on several parts working together
    * core [minimizer](https://github.com/rezer0dai/LibAFL/blob/smb2/libafl/src/corpus/rotator.rs#L146-L308) logic
    * [passtrough](https://github.com/rezer0dai/LibAFL/blob/smb2/libafl/src/feedbacks/map.rs#L655-L679) **all fuzzed inputs** into minimizer
    * add removing of inputs from corpus, [imidiatelly](https://github.com/rezer0dai/LibAFL/blob/smb2/libafl/src/fuzzer/mod.rs#L374-L380) if RAFLB does not use it or [postpone](https://github.com/rezer0dai/LibAFL/blob/smb2/libafl/src/stages/power.rs#L219-L251) until it is OK to do so
    * [heavy modification](https://github.com/rezer0dai/bananafzz/blob/smb2/tools/patches/libafl-bijon-rotator-bfl.patch#L1688-L1913) ondisk [corpus logic](https://github.com/rezer0dai/LibAFL/blob/smb2/libafl/src/corpus/ondisk.rs#L247-L343) to allow progressive add + remove of entries alongside with RAFLB ( worth to check if it is OK to easier logic a bit based on certain constrains which are now, in this state of implementation, clear )

## wip
for now established LibAFL + bananafzz cooperation, need setup more comprehensive toy example of usage. Currently testing on Super Mario Bros 2, though I dont consider it to be best example :) meanwhile to setup your own tests, you can use [cargo-libafl](https://github.com/AFLplusplus/cargo-libafl) or [StdFuzzer](https://github.com/AFLplusplus/StdFuzzer) and hack it [similiar](https://github.com/rezer0dai/bananafzz/tree/smb2/tools/patches) way of this :

```rust
// ...
use libafl::{
    // ...
    feedbacks::RotationAflMapFeedback as MaxMapFeedback,//AflMapFeedback, 
    mutators::bsched::banana_mutations,//scheduled::havoc_mutations,
    corpus::IndexesRotatingCorpusScheduler as IndexesLenTimeMinimizerCorpusScheduler,
    // ...
};

// ...

/// The main fn, `no_mangle` as it is a C symbol
#[no_mangle]
pub fn libafl_main() {
// ...
    let monitor = TuiMonitor::new(
        format!("Bananaized Fuzzy Loop (BFL) <LibAFL's StdFuzzer v{} + bananafzz> v0.1", VERSION),
        !opt.disable_unicode,
    );
// ...
    let mut run_client = |state: Option<StdState<_, _, _, _, _>>, mut mgr, _core_id| {
// ...
        let edges_observer = HitcountsMapObserver::new(
            BijonObserver::new(1000));
// ...
        let feedback_state = MapFeedbackState::with_observer(&edges_observer);
        let feedback = feedback_or!(
            MaxMapFeedback::new_tracking(&feedback_state, &edges_observer, true, false),
// ...
        );

        let objective = feedback_or!(CrashFeedback::new(), TimeoutFeedback::new());

        // If not restarting, create a State from scratch
        let mut state = state.unwrap_or_else(|| {
            StdState::new(
                // RNG
                StdRand::with_seed(current_nanos()),
// ** THIS is actual essential, as we are using IndexesRotatingCorpusScheduler based on ondisk.rs patches !!
                OnDiskCorpus::new(corpus_dir.clone()).unwrap(),
                OnDiskCorpus::new(output_dir.clone()).unwrap(),
                tuple_list!(feedback_state),
            )
    });
// ..
        let (bananas, banana) = banana_mutations();
        let poc_mem = unsafe { banana.read().unwrap().poc_mem() };
        let mutator = StdMOptMutator::new(&mut state, bananas, 5)?;
// ..
        let power = PowerMutationalStage::new(mutator, PowerSchedule::FAST, &edges_observer);
        let scheduler =
            IndexesLenTimeMinimizerCorpusScheduler::new(PowerQueueCorpusScheduler::new());

        // A fuzzer with feedbacks and a corpus scheduler
        let mut fuzzer = StdFuzzer::new(scheduler, feedback, objective);

        let mut harness = |input: &BytesInput| {
            let target = input.target_bytes();
            let buf = target.as_slice();
            if 0 == libfuzzer_test_one_input(poc_mem, buf) {
                ExitKind::Ok
            } else { ExitKind::BflErrorRepro }
        };
//..
    }
//..
}

```

## eta when
I will make blog post when I will polish it on real world target, and have version matured enough to showcase why eating LibAFL cpu time over bananafzz is worhwhile to do.

## ps
go get some bananas, they are good :)


