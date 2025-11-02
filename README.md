



<img src="https://github.com/Software-Knife-and-Tool/mu/blob/main/.github/mu.png?raw=true" width="20%" height="%20">

# *mu* - system programming environment

### Under heavy development 

###### version 0.2.13

*mu* is a Lisp-idiomatic functionally-oriented interactive environment for system programming in the Rust ecosystem. It is targeted to low-resource persistent POSIX environments.

*mu* is a Lisp-1 namespaced programming language with Common Lisp idioms and macro system.

While *mu* has much in common with Scheme, it is meant to be familiar to traditional LISP programmers.

*mu* is a 2-LISP, which gives it flexibility in implementation and subsequent customization. A small native code runtime kernel supports system classes, function application, heap/system image management, garbage collection, and an FFI framework.

Subsequent layers based on the runtime offer advanced features.



#### Recent changes

------



- runtime constant compilation

  

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



- *mu*, a small, configurable runtime library and language
- *mu-sys*, minimal POSIX command suitable for containers
- *codegen*, a native code compiler
- *forge* , a cargo-like development and packaging tool
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
% forge doc
```

and will end up in ```doc/rustdoc```. The ``doc/rustdoc/mu``  subdirectory contains the starting ```index.html```.

The *mu* reference documentation is a collection of *markdown* files in `doc/reference`. To generate the documentation, you will need the *pandoc* utility, see *https://pandoc.org*

Once built, the *html* for the *reference* material is installed in *doc/reference/html*, starting with *index.html*.



#### About the Release

------

 The release is installed in `/opt/mu`. 

```
/opt/mu
├── bin
├── doc
│   └── html
├── lib
│   ├── core
│   ├── fasl
│   ├── image
│   └── listener
└── modules
    ├── common
    │   ├── describe
    │   └── metrics
    └── prelude
        └── repl
```

If you want to install a release from the github repository

```
cat mu-x.y.z.tgz | (cd / ; sudo tar --no-same-owner -xzf -)
```

The `/opt/mu` directory is hardwired into several tools and the release mechanism, changing it would require significant alteration of parts of the system. 

  

#### Building the *mu* system

------

```
version 0.2.10 is built with rustc 1.89.0
version 0.2.11 is built with rustc 1.90.0
version 0.2.12 is built with rustc 1.90.0
```

The *mu* runtime is a native code program that must be built for the target CPU architecture. The runtime build system requires only a `rust` development environment, `rust-fmt`, `clippy` and the  GNU `make` utility. Other development tools like  `valgrind` are optional.

Tests, performance, tools, and regression metrics require some version of `python 3`.

```
git clone https://github.com/Software-Knife-and-Tool/mu.git
```

After cloning the *mu* repository, the *mu* system can be built and installed with the supplied makefile. The *world* target builds a release version of the system and the *forge* development tool.  `make` with no arguments prints the available targets. 

```
% make world
```

Having built the distribution, install it in `/opt/mu`.

```
% sudo make install
```

Having built and installed `mu`,  establish the current directory as a `forge`  workspace.

```
% forge workspace init
```

Subsequent builds and packaging of the system are facilitated with *lade*. See the *Tools* section below for usage instructions.

Note: the *forge* and *makefile* installation mechanisms do not remove the installation directory before writing it and changes to directory structure and files will accumulate.



#### Features

------

The *mu* runtime supports conditional compilation of a variety of features. 

Currently supported features by namespace:

```
 default = [ "env", "core", "system" ]
 
 feature/core:			core process-mem-virt process-mem-res
 						process-time time-units-per-sec delay
 feature/env:			env heap-info heap-size heap-room cache-room
 feature/system:		uname shell exit sysinfo
 feature/instrument:    instrument-control

```

The *sysinfo* feature is disabled on *macOS* builds.



#### Tools

------

The *mu* distribution includes tools for configuring and development of the system. 

The  *forge* command is part of a release, found at `/opt/mu/bin/forge`.

```
Usage: forge 0.0.18 command [option...]
  command:
    help                               ; this message
    version                            ; forge version

    workspace init | env               ; manage workspace
    build     release | profile | debug
                                       ; build mu system, release default
    bench     base | current | report | clean [--ntests=number] [--all]
                                       ; benchmark test suite
    regression                         ; regression test suite
    symbols   reference | crossref | metrics [--namespace=name]
                                       ; symbol reports, namespace 
                                       ; defaults to mu
    install                            ; (sudo) install mu system-wide
    clean                              ; clean all artifacts
    commit                             ; fmt and clippy, pre-commit checking

  general options:
    --verbose                          ; verbose operation
    --recipe                           ; show recipe
```

`forge` is styled after `cargo` and fulfills many of the same needs. While the help message should be relatively explanatory, the general development workflow is something like this. Note that in this version **=** is mandatory for options with arguments.

Before making any changes, you will want to establish a performance baseline.

```
 forge bench base
```

As you make changes, you can verify correctness and note any performance regressions.

Deviations of 20% or so in timing are normal, any changes in storage consumption or a persistent change in timing of an individual test significantly above 20% should be examined.

```
 forge build release	# build the release version 
 forge test				# run the regression tests
 forge bench current	# benchmark current build and print results
```

The `symbols` command prints a list of the symbols in various namespaces and their types.

Profiling is nascent and will be expanded in future releases. 



The *mu* distribution includes a tool for running and interacting with the system. 

The  *listener* binary is part of a release, found at `/opt/mu/bin/listener`.

*listener*  has no command line arguments. It is configured by an optional JSON file, *.listener*, which is expected to be in either the current directory or the user's home directory. The *config* argument supplies a *mu* environment configuration string (see **System Configuration** for details), and the *rc* argument supplies the name of a file to load on startup. Examples of both of these files can be found in `/opt/mu/lib/listener`.

*listener* will run without either of *.listener* or an rc file.

```
{
    "config": {
    	"pages": "2048",
    	"gc-mode": "auto"
    },
    "namespace": "mu",
    "rc": "mu-listener.rc"
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

Metrics include the average amount of time (in microsconds) taken for an individual test and the number of objects allocated by that test. Differences between runs in the same installation can be in the 20% range. Any changes in storage consumption or a large (20% or greater) increase in test timing warrant examination. Note: `ntests` of 50 seem to demonstrate least variation between runs of identical *mu* binaries.

```
forge bench current
```

 The **NTESTS** environment variable (defaults to 20) controls how many passes are included in a single test run.

On a modern Core I7 CPU at 3+ GHz, the default performance tests take around 10 minutes of elapsed time. 

The *makefile* version of the performance and regression tests are included in upcoming releases, but are considered legacy and will eventually be deprecated.

See the *forge* usage section for the equivalent *forge*  `bench` command.

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
% make -C tests/performance help

--- performance options
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

The *mu* binaries and libraries are installed in `/opt/mu`. The `bin` directory contains the binaries for running the system.

```
forge	development system tool
mu-sys		runtime binary
listener	runtime binary, stdio listener
mu-ld		image loader
mu-exec     image executor
mu-server	server runtime, socket listener
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

An interactive session for the extended *mu* system is invoked by the `mu-listener` command.

*rlwrap* makes the *listener* repl much more useful, with command history and line editing.

```
% alias ,listener='rlwrap listener'
```

Depending on your version of *rlwrap*, *,listener* may exhibit odd echoing behavior. Adding

```
set enable-bracketed-paste off
```

to your `~/.inputrc` may help. If you want to run the prelude listener as part of an interactive session:



#### System Configuration

------

The *mu* runtimes can be configured to use a variable number of system resources, currently the number of pages of memory allocated to the heap at startup. The behavior of the garbage collector can also be specified, though garbage collection control is still mostly unimplemented.

 The *-c* option to the various runtimes is a JSON string of named attribute values:

```
npages:	number				pages of virtual memory for the heap
gc-mode: "none" | "auto"	how the garbage collector operates
```

Usage: (*mu-server*, *mu-ld*, and *mu-exec* have similar options)

```
mu-sys -c '{ "pages": 256, "gc-mode": "none" }'		
									256 heap pages, garbage collection disabled
mu-sys -c '{ "pages": 1024, "gc-mode": "auto" }'
									default configuration
```
