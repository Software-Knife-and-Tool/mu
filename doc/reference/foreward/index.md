---
title: mu/eth - a lightweight LISP environment for Rust
---

### **Foreward**

<hr>

Welcome to *mu/eth*, a functional forward LISP programming environment for the Rust language ecosystem.

Dynamic languages for low overhead environments must be lightweight, but developing applications in the target environment can be painful. *mu/eth* is a general development and execution environment for containers, shells, and other utilities that need small installation and runtime footprints.

*mu/eth* was designed to operate effectively on *Raspberry Pi* class machines and containers, and consists of a small native-code kernel runtime and a set of layered libraries in source form.

The *mu/eth* runtime is a native code library that directly interprets *mu* kernel code. A great deal of useful work can be done directly with the limited kernel language, though it lacks the niceties of a more traditional LISP programming environment. More advanced language features require the use of the included *prelude* library. 

The *mu* library provides traditional LISP forms and a collection of functional programming features like pattern matching and monads. *mu/eth*, *prelude*, and *preface* code are human-readable and can be easily modified and extended.

Questions? You can contact the author at putnamjm.design@gmail.com.

<div style="page-break-after: always"></div>

### **Table of Contents**

------

**Foreward** ........................................................................................................</br>
**mu** ....................................................................................................................</br>
**prelude** ............................................................................................................</br>
2.1 *about* ........................................................................................................... [2.1](2-1.prelude.html)</br>
2.2 *reader* ......................................................................................................... [2.2](2-2.reader.html)</br>
2.3 *compiler* ...................................................................................................... [2.3](2-3.compiler.html)</br>
2.4 *lambda* ........................................................................................................ [2.4](2-4.lambda.html)</br>
2.5 *functions* ...................................................................................................... [2.5](2-5.functions.html)</br>
2.6 *macros* ......................................................................................................... [2.6](2-6.macros.html)</br>
2.7 *sequences* ..................................................................................................... [2.7](2-7.sequences.html)</br>
2.8 *exceptions* .................................................................................................... [2.8](2-8.exceptions.html)</br>
2.9 *lists* ............................................................................................................... [2.9](2-9.lists.html)</br>
2.10 *streams* ....................................................................................................... [2.10](2-10.streams.html)</br>
2.11 *utilities* ......................................................................................................... [2.11](2-11.utilities.html)</br>
2.12 *prelude symbols* .......................................................................................... [2.12](2-12.prelude-symbols.html)</br>
**mu** ................................................................................................................</br>
**mu/eth** ..........................................................................................................</br>
**system** ................................................................................................................</br>
**Appendices** ........................................................................................................</br>



