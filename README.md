



<img src="https://github.com/Software-Knife-and-Tool/mu/blob/main/.github/mu.png?raw=true" width="20%" height="%20">

# *mu* - system programming environment

### Under heavy development 

###### version 0.2.9

*mu* is a Lisp-idiomatic functionally-oriented interactive environment for system programming in the Rust ecosystem. It is targeted to low-resource persistent POSIX environments.

*mu* is a Lisp-1 namespaced programming language with Common Lisp idioms and macro system.

While *mu* has much in common with Scheme, it is meant to be familiar to traditional LISP programmers.

*mu* is a 2-LISP, which gives it flexibility in implementation and subsequent customization. A small native code runtime kernel supports system classes, function application, heap/system image management, garbage collection, and an FFI framework.

Subsequent layers based on the runtime offer advanced features.



#### Recent changes

------

- env feature
- mcore command line utility

#### Rationale

------

Functional languages bring us closer to a time when we can automatically prove our programs are correct. As systems get more complex, we'll need increased assurance that their various components are reliable. Modern programming concepts do much to underpin reliability and transparency.

*mu* attempts to express modern programming concepts with a simple, familiar syntax. The venerable Common Lisp macro system helps the system designer create domain specific languages.

*Lisps* are intentionally dynamic, which has selected against them for use in production environments. Statically-typed languages produce systems that are hard to interact with and impossible to change *in situ*. Few languages in use today have adequate meta-programming facilities. We need systems that can we reason about and can supplement themselves.

Current systems tend to be large and resource-hungry. We need lean systems that can do useful work in low resource environments and are flexible enough to evolve to meet new demands. Current systems have runtimes measured in days, if for no other reason than improving them requires a complete reinstall. A dynamic system can have a runtime measured in months or years.

Evolutionary response to change is the only defense a system has against obsolescence.

Most of our core computational frameworks are built on static systems and are fragile with respect to change. Such systems tend to be disposable. Lightweight dynamic systems designed for persistence are the next step.



#### Project Components and Goals

------



- *mu*, a small, configurable runtime library amd language
- *mu-sys*, minimal POSIX runtime suitable for containers
- *codegen*, a native code compiler
- *mux* , a cargo-like development and packaging tool
- small and simple installation
- add interactivity and extensibility to application implementations
- Rust FFI system
- mostly Common Lisp syntax
- Common Lisp macro system
- resource overhead equivalent to a UNIX shell
- futures for async and non-blocking I/O



#### State of the *mu* system

------

*mu* is a work in progress and under heavy development.

*mu* runtime builds are targeted to:

- x86-64 and AArch-64 Linux distributions
- x86-64 WSL
- Docker Ubuntu and Alpine containers

Current releases on github are Linux x86-64, other architectures will follow.

Portability, libraries, deployment, documentation, and garbage collection are currently the top priorities.



#### About *mu*

------

*mu* is an immutable, namespaced Lisp-1 that borrows heavily from *Scheme*, but is more closely related to the Common Lisp family of languages. *mu* syntax and constructs will be familiar to the traditional Lisp programmer. 

*mu* leans heavily on functional programming principles.

The *mu* runtime kernel is written in mostly-safe `rust` (the system image/heap facility *mmaps* a file and random user selected features may have unsafe implementations.)

The runtime implements 64 bit tagged pointers, is available as a crate, and extends a Rust API for embedded applications. The runtime is primarily a resource allocator and evaluator for the *mu-runtime* kernel language. *mu-runtime* provides the usual fixed-width numeric types, lists, fixed-arity lambdas, simple structs, LISP-1 symbol namespaces, streams, and specialized vectors in a garbage collected environment.

The *mu* 2-LISP system is organized as a stack of compilers, culminating in the *codegen* native code compiler/system builder.

The *core* library provides *rest* lambdas, *closures*, expanded types, *macros*, and a reader/compiler for those forms.

Optional libraries provide a variety of enhancements and services, including Common LISP macros and binding special forms.




#### Viewing the documentation

------

*mu* and *core* reference cards can be found in ```doc/refcards``` in a variety of formats. They document the *mu*  and *core* namespaces, the runtime API, and options for running *mu-sys*.

The *mu* crate rustdoc documentation can be generated by

```
% mux doc
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

```
version 0.2.5 is built with rustc 1.86.0
version 0.2.6 is built with rustc 1.86.0
version 0.2.7 is built with rustc 1.87.0
version 0.2.8 is built with rustc 1.88.0
```

The *mu* runtime is a native code program that must be built for the target CPU architecture. The runtime build system requires only a `rust` development environment, `rust-fmt`, `clippy` and the  GNU `make` utility. Other development tools like  `valgrind` are optional.

Tests, performance, tools, and regression metrics require some version of `python 3`.

```
git clone https://github.com/Software-Knife-and-Tool/mu.git
```

After cloning the *mu* repository, the *mu* system can be built and installed with the supplied makefile. The *world* target builds a release version of the system and the *mux* development tool.  `make` with no arguments prints the available targets. 

```
% make world
```

Having built the distribution, install it in `/opt/mu`.

```
% sudo make install
```

Having built and installed `mu`,  establish the current directory as a `mux`  workspace. In releases prior to 0.1.82, the make `world` target does this automatically. In 0.1.82 and later releases:

```
% mux init
```

Subsequent builds and packaging of the system are facilitated with *mux*. See the *mux* section below for usage instructions.

Note: the *mux* and *makefile* installation mechanisms do not remove the installation directory before writing it and changes to directory structure and files will accumulate.



#### Features

------

As of 0.0.40, the *mu* runtime supports conditional compilation of a variety of features. 

Currently supported features by namespace:

```
 default = [ "env", "std", "sysinfo", "core", "prof", "nix" ]
 
 mu/core:		core env process-mem process-time time-units-per-sec delay
 mu/env:		env heap-stat heap-size 
 mu/nix:     	uname
 mu/prof:    	prof-control
 mu/std:     	command exit
 mu/sysinfo: 	sysinfo
```

The *sysinfo* feature is disabled on *macos* builds. The *semispace* feature is not yet functional.



#### Tools

------

As of 0.1.76, the *mu* distribution includes tools for configuring and development of the system. 

The  *mux* binary is part of a release, found at `/opt/mu/bin/mux`.

The *mux* tool provides these utilities:

```
Usage: mux 0.0.16 command [option...]
  command:
    help                               ; this message
    version                            ; mux version
    init                               ; init
    env                                ; print development environment
    build     release | profile | debug
                                       ; build mu system, release is default
    image     build --out=path | 
              [--image=path | -config=config]
              *[--load=path | --eval=sexpr]] | view --image=path
                                       ; manage heap images
    symbols   reference [--module=name] |
              crossref [--module=name]  |
              metrics [--module=name]
                                       ; symbol reports, default to mu
    install                            ; (sudo) install mu system-wide
    clean                              ; clean all artifacts
    commit                             ; fmt and clippy, pre-commit checking
    test                               ; regression test suite
    bench     base | current | footprint [--ntests=number]

  general options:
    --verbose                          ; verbose operation
```

`mux` is styled after `cargo` and fulfills many of the same needs. While the help message should be relatively explanatory, the general development workflow is something like this. Note that in this version **=** are mandatory for options with arguments.

Before making any changes, you will want to establish a performance baseline.

```
 mux bench base --ntests=1
```

As you make changes, you can verify correctness and note any performance regressions. Deviations of 20% or so in timing are normal, any changes in storage consumption or a persistent change in timing of an individual test significantly above 20% should be examined.

```
 mux build release			# build the mu release version 
 mux test				# run the regression tests
 mux bench current --ntests=1	# benchmark the current build and print results
```

The `symbols` command prints a list of the symbols in various namespaces and their types.

Profiling is nascent and will be expanded in future releases. 



As of 0.2.9, the *mu* distribution includes a tool for running and interacting with the system. 

The  *mcore* binary is part of a release, found at `/opt/mu/bin/mcore`.

*mcore*  has no command line arguments, it is configured by a JSON file, *.mcore*, which is expected to be in either the current directory or the user's home directory. The *config* argument supplies a *mu* environment configuration string (see System Configuration for details), and the *rc* argument supplies the name of a file to load on startup. Examples of both of these files can be found in `/opt/mu/lib/mcore`.

*mcore* will run without either of *.mcore* or an rc file.

```
{
    "config": "npages: 2048",
    "rc": "mcorerc"
}
```



#### Testing

------

The distribution includes a test suite, which should be run after every interesting change. The test suite consists of a several hundred individual tests roughly separated by namespace.

Failures in the *mu* tests are almost guaranteed to cause complete failure of subsequent tests.

```
% make tests/base
% make tests/current
% make tests/report
```

The `report` target produces a human readable test report. 

The `tests` makefile has additional facilities for development. The `help` target will list them.

```
% make -C tests/regression help

regression test makefile -----------------

--- test options
    commit - create test summary
    test - run single test group in $NS/$TEST
    summary - run all tests in all namespaces and print summary
    
```

#### Performance metrics

------

Metrics include the average amount of time (in microsconds) taken for an individual test and the number of objects allocated by that test. Differences between runs in the same installation can be in the 10% range. Any changes in storage consumption or a large (10% or greater) increase in test timing warrant examination. Note: As of 0.2.0, the performance runs can take up to half an hour. It would be best to limit performance testing to `NTESTS=1` or 

```
mux bench current --ntests=1
```

 The **NTESTS** environment variable (defaults to 20) controls how many passes are included in a single test run.

On a modern Core I7 CPU at 3+ GHz, the performance tests take around 30 minutes of elapsed time. See the *mux* usage section for the equivalent *mux*  `bench` command.

```
% make -C tests/performance base
% make -C tests/performance current
% make -C tests/performance report
% make -C tests/performance commit
```

The `base` target produces a performance run and establishes a baseline. The `current`  target produces a secondary performance run. The `report` target produces a human-readable diff between `base` and `current`.  Normally, you'd run a baseline, make some changes, and then do a current run to see if there were any regressions.

The `performance` makefile has additional facilities for development, including reporting on individual tests. The `help` target will list them. 

In specific, a summary of significant performance changes (differences in measured resource consumption and/or a large difference in average test time between the current summary and the baseline summary.)

```
% make -C tests/performance commit
```

produces a report of the differences between the current summary and the established baseline. The *commit* target reports on any change in storage consumption between the baseline and the current summary.

The  `performance`  makefile offers some development options.

```
% make -C tests/regression help

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

The *mu* binaries and libraries are installed in `/opt/mu`. The `bin` directory contains the binaries and shell scripts for running the system. The library sources are included in `lib`. *mux*  has an equivalent facility in the *repl* command.

```
mu-sys		runtime binary
mu-sh		runtime binary, stdio listener
mu-ld		image loader
mcore		interactive runtime, stdio listener
mu-exec     	image executor
mu-server	server runtime, socket listener
mu		shell script for loading *core*. runs *mu-sh* listener
```


```
OVERVIEW: mu-sys - posix platform mu exec command
USAGE: mu-sys [options] [file...]

runtime: x.y.z: [-h?svcelq] [file...]
OPTIONS:
  -v                   print version string and exit
  -l SRCFILE           load SRCFILE in sequence
  -e SEXPR             evaluate SEXPR and print result
  -q SEXPR             evaluate SEXPR quietly
  -c name:value[,...]  environment configuration  	    
  [file ...]           load source file(s)
```

An interactive session for the extended *mu* system is invoked by the `mu` shell script. The *mu* repl does not display a prompt. *mu-sys* configures the library to catch SIGINT and generate a `:sigint` exception rather than abort the process.

```
% /opt/mu/bin/mu 
```

(the *prelude* namespace is currently under development.)

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

The *mu* runtimes can be configured to use a variable number of system resources, currently the number of pages of memory allocated to the heap at startup. The behavior of the garbage collector can also be specified, though garbage collection control is still mostly unimplemented. The *-c* option to the various runtimes is a string of named attribute values:

```
npages			number									pages of virtual memory for the heap
page-size		number									number of bytes in a page
gc-mode			{ none, auto, demand }	how the garbage collector operates
heap-type		{ semispace, bump }			heap type, defaults to bump
```

Usage: (*mu-server*, *mu-ld*, and *mu-exec* have similar options)

```
mu-sys -c "npages:256,gc-mode:none"	256 heap pages, garbage collection disabled
mu-sys -c "npages:1024,gc-mode:auto"	default configuration

mu --config="npages:4096, gc-mode:demand, heap-type:bump"
					4096 pages, garbage collection runs on demand, bump allocator 
```
