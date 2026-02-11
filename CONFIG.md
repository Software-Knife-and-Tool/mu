# *mu* - environment configuration 

###### version 0.2.15

#### Environment Configuration

------

An individual *mu* runtime environment can be configured to use a variable number of system resources,
currently the number of 4k pages of memory allocated to the heap at startup. The behavior of the garbage
collector can also be specified, though garbage collection control is still mostly unimplemented.

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
