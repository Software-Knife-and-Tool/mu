



<img src="https://github.com/Software-Knife-and-Tool/mu/blob/main/.github/mu.png?raw=true" width="20%" height="%20">

# *mu* - a system programming environment


### Under heavy development

*mu* is a Lisp-idiomatic functionally-oriented interactive environment for system programming in the Rust ecosystem. It is targeted to low-resource persistent POSIX environments.

*mu* is a Lisp-1 namespaced programming language with Common Lisp idioms and macro system.

While *mu* has much in common with Scheme, it is meant to be familiar to traditional LISP programmers.

*mu* is a 2-LISP, which gives it flexibility in implementation and subsequent customization. A small native code runtime kernel supports system classes, function application, heap/system image management, garbage collection, and an FFI framework.

Subsequent layers based on the runtime offer advanced features.



#### Rationale

------

Functional languages bring us closer to a time when we can automatically prove our programs are correct. As systems get more complex, we'll need increased assurance that their various components are reliable. Modern programming concepts do much to underpin reliability and transparency.

*mu* attempts to express modern programming concepts with a simple, familiar syntax. The venerable Common Lisp macro system helps the system designer create domain specific languages.

*Lisps* are intentionally dynamic, which has selected against them for use in production environments. Statically-typed languages produce systems that are hard to interact with and impossible to change *in situ*. Few languages in use today have adequate meta-programming facilities. We need systems that can we reason about and can supplement themselves.

Current systems tend to be large and resource-hungry. We need lean systems that can do useful work in low resource environments and are flexible enough to evolve to meet new demands. Current systems have runtimes measured in days, if for no other reason than improving them requires a complete reinstall. A dynamic system can have a runtime measured in months or years.

Evolutionary response to change is the only defense a system has against obsolescence.

Most of our core computational frameworks are built on static systems and are fragile with respect to change. Such systems tend to be disposable. Lightweight dynamic systems designed for persistence are the next step.



#### Project Goals

------

- *mu*, a functional forward system language
- *mu/lib*, a small, configurable runtime library
- *mu-sys*, minimal POSIX runtime suitable for containers
- *mu/codegen*, a native code compiler
- small and simple installation, no external dependencies
- add interactivity and extensibility to application implementations
- Rust FFI system
- mostly Common Lisp syntax
- resource overhead equivalent to a UNIX shell
- minimal external crate dependencies
- futures for async and non-blocking I/O



#### State of the *mu* system

------

*mu* is a work in progress.

The runtime builds with rust 1.70 or better. *mu* runtime builds are targeted to:

- x86-64 and AArch-64 Linux distributions
- x86-64 and M-series MacOs X
- x86-64 WSL
- Docker Ubuntu and Alpine containers

Current releases on github will are for Linux x86-64, other architectures will follow.

Portability, libraries, deployment, documentation, and garbage collection are currently the top priorities.



#### About *mu*

------

*mu* is an immutable, namespaced Lisp-1 that borrows heavily from *Scheme*, but is more closely related to the Common Lisp family of languages. *mu* syntax and constructs will be familiar to the traditional Lisp programmer. 

*mu* leans heavily on functional programming principles.

The *mu* runtime kernel, *lib*, is written in mostly-safe `rust` (the system image/heap facility *mmaps* a file, which is an inherently unsafe operation.)

The runtime implements 64 bit tagged pointers, is available as a library, and  extends a Rust API for embedded applications. the runtime is primarily an evaluator for the *mu/lib* kernel language. *mu/lib* provides the usual fixed-width numeric types, lists, fixed-arity lambdas, simple structs, LISP-1 symbol namespaces, streams, and specialized vectors in a garbage collected environment.

The *mu* 2-LISP system is organized as a stack of compilers, culminating in the *mu/codegen* native code compiler/system builder.

The *prelude* library provides *rest* lambdas, closures, expanded struct types, macros, and a reader/compiler for those forms.

Optional libraries provide a variety of enhancements and services, including Common LISP macros and binding special forms.


#### Viewing the documentation

------

A handy *mu/lib* reference card can be found in ```doc/refcards``` in a variety of formats. It documents the *lib* namespace, the runtime API, and options for running *mu-sys*.

The *lib* crate rustdoc documentation can be generated by

```
% make doc
```

and will end up in ```doc/rustdoc```. The ``doc/rustdoc/mu``  subdirectory contains the starting ```index.html```.

The *mu* reference documentation is a collection of *markdown* files in `doc/reference`. To generate the documentation, you will need the *pandoc* utility, see *https://pandoc.org*

Once built, the *html* for the *reference* material is installed in *doc/reference/html*, starting with *index.html*.

#### Installing a release

------

If you want to install a release from the github repository

```
cat mu-x.y.z.tgz | (cd $ROOT ; sudo tar --no-same-owner -xzf -)
```

where `$ROOT` is the intended destination directory. The `mu.sh` scripts assumes `/opt`, if you want it elsewhere you will have to make some modifications. If I get a wild hair, I'll put this in  the environment at some point.

  

#### Building the *mu* system

------

The *lib* runtime is a native code program that must be built for the target CPU architecture. The runtime build system requires only a `rust` compiler, `rust-fmt`, `clippy` and the  GNU `make` utility. Other development tools like  `valgrind` are optional.

Tests and regression metrics require some version of `python 3`.

```
git clone https://github.com/Software-Knife-and-Tool/mu.git
```

After cloning the *mu* repository, the *rust* system can be built and installed with the supplied makefile.

```
% make release
% make debug
```

Having built the distribution, install it in `/opt/mu`.

```
% sudo make install
```

`make` with no arguments prints the available targets.

If you want to repackage *mu* after a change to the library sources:

```
% make dist
```

and then reinstall.

Note: the installation mechanism does not remove the installation directory before writing it and changes to directory structure and files will tend to accumulate. The make *uninstall* target will remove that if desired.

```
% sudo make uninstall
```



#### Features

------

As of 0.0.40, the *lib* runtime supports conditional compilation of a variety of features. 

Currently supported features by namespace:

```
 default = [ "std", "sysinfo", "nix" ]
 
 nix:     uname
 std:     command, exit
 sysinfo: sysinfo
```

The *sysinfo* feature is disabled on *macos* builds.



#### Testing

------

The distribution includes a test suite, which should be run after every interesting change. The test suite consists of a several hundred individual tests roughly separated by namespace.

Failures in the *lib* tests are almost guaranteed to cause complete failure of subsequent tests.

```
% make tests/summary
% make tests/commit
```

The `summary` target produces a human readable test report. This summary will be checked into the repo at the next commit.

 The `commit` target will produce a diff between the current summary and the repo summary.

The `tests` makefile has additional facilities for development. The `help` target will list them.

```
% make -C tests help

--- test options
    cargo - run rust tests
    namespaces - list namespaces
    commit - create test summary
    tests - tests in $NS
    mu | prelude - run all tests in namespace, raw output
    test - run single test in $NS/$TEST
    summary - run all tests in all namespaces and print summary
```



#### Performance regression metrics

------

Metrics include the average amount of time (in microsconds) taken for an individual test and the number of objects allocated by that test. Differences between runs in the same installation can be in the 10% range. Any changes in storage consumption or a large (10% or greater) increase in test timing warrant examination.

The **NTESTS** environment variable (defaults to 20) controls how many passes are included in a single test run.

On a modern Core I7 CPU at 3+ GHz, the default regression tests take approximately four minutes of elapsed time for both the *mu* and *prelude* namespaces.

```
% make -C metrics/regression base
% make -C metrics/regression current
% make -C metrics/regression report
% make -C metrics/regression commit
```

The `base` target produces a performance run and establishes a base line. The `current`  target produces a secondary performance run. The current summary will be checked into the repo as the base at the next commit. The `report` target produces a human-readable diff between `base` and `current`. 

The `regression` makefile has additional facilities for development, including reporting on individual tests. The `help` target will list them. 

In specific, a summary of significant performance changes (differences in measured resource consumption and/or a large difference in average test time between the current summary and the baseline summary.) Timing metrics are heavily CPU/clock speed dependent.

```
% make -C metrics/regression commit
```

produces a report of the differences between the current summary and the established baseline. The *commit* target reports on any change in storage consumption between the baseline and the current summary, and timing changes greater than 20% for any individual test. `commit` also establishes the `current` report as the new baseline in preparation for a commit to the repo.

For convenience, the *mu* Makefile provides:

```
% make regression/base		# establish a baseline report
% make regression/current	# produce a secondary report
% make regression/report	# diff baseline and current
% make regression/commit	# diff baseline and current, prepare for commit
```

The  `regression`  makefile offers some development options.

```
% make -C metrics/regression help

--- regression options
    namespaces - list namespaces
    list - list tests in $NS
    $NS - run all tests in namespace, unformatted output
    base - run all tests in all namespaces, establish baseline report
    current - run all tests in all namespace, establish current report
    commit - compare current with base, promote current to base
    report - compare current report with base report
    metrics - run tests and verbose report
```



#### Running the *mu* system

------

The *mu* binaries and libraries are installed in `/opt/mu`. The `bin` directory contains the binaries and shell scripts for running the system. The library sources are included in `lib`.

```
mu-sys		shell runtime binary, stdio listener
mu-ld		image loader
mu-server	server runtime, socket listener
mu		shell script for running the prelude listener with mu-sys
```


```
OVERVIEW: mu-sys - posix platform mu shell command
USAGE: mu-sys [options] [file...]

runtime: x.y.z: [-h?psvcelq] [file...]
OPTIONS:
  -h                   print this message
  -?                   print this message
  -v                   print version string and exit
  -p                   pipe mode, no repl
  -l SRCFILE           load SRCFILE in sequence
  -e SEXPR             evaluate SEXPR and print result
  -q SEXPR             evaluate SEXPR quietly
  -c name:value[,...]  environment configuration  	   
  [file ...]           load source file(s)
```

An interactive session for the extended *mu* system is invoked by the `mu` shell script. The *mu* repl does not display a prompt.

```
% /opt/mu/bin/mu 
```

If you want to exercise the *prelude* repl,

```
% /opt/mu/bin/mu --eval='(prelude:repl)'
prelude>
```

`:h` will print the currently available repl commands. Forms entered at the prompt are evaluated and the results printed. The prompt displays the current namespace. *rlwrap* makes the *mu* and *runtime* repls much more useful, with command history and line editing.

```
% alias ,mu-sys='rlwrap mu'
```

Depending on your version of *rlwrap*, *mu* may exhibit odd echoing behavior. Adding

```
set enable-bracketed-paste off
```

to your `~/.inputrc` may help. If you want to run the prelude listener as part of an interactive session:

```
alias ,mu-repl='rlwrap mu --eval='\''(prelude:repl)'\'''
```



#### System Configuration

------

The *mu* runtimes can be configured to use a variable number of system resources, currently the number of pages of memory allocated to the heap at startup. The behavior of the garbage collector can also be specified, though garbage collection is still mostly unimplemented. The *-c* option to the various runtimes is a string of named attribute values:

```
npages			number of pages of virtual memory for the heap
gcmode			{ none, auto, demand } how the garbage collector operates
```

Usage: (*mu-server*, *mu-ld*, and *mu-exec* have similar options)

```
mu-sys -c "npages:256,gcmode:none"	256 heap pages, garbage collection disabled
mu-sys -c "npages:1024,gcmode:auto"	default configuration

mu --config="npages:4096,gcmode:demand"
					4096 pages, garbage collection runs on demand 
```

Tests show that currently (and as of 0.0.23) 256 4k pages is about the minimum you could expect to load the *preface* library and run the listener. Any significant consing will likely run out of heap space in short order.
