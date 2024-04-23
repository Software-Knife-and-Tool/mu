###  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
###  SPDX-License-Identifier: MIT
###
import json
import os
import sys
import subprocess

ntests = sys.argv[1]

mu_cmd = '../../dist/mu-sys'
time_cmd = '/usr/bin/time'
format = '"%S %U %e %M %w %Z"'

def stats():
    proc = subprocess.Popen([
        time_cmd,
        '-f',
        format,
        mu_cmd,
        '-p',
        '-l', '../../src/prelude/prelude.l',
        '-l', '../../src/prelude/break.l',
        '-l', '../../src/prelude/compile.l',
        '-l', '../../src/prelude/ctype.l',
        '-l', '../../src/prelude/describe.l',
        '-l', '../../src/prelude/environment.l',
        '-l', '../../src/prelude/exception.l',
        '-l', '../../src/prelude/fixnum.l',
        '-l', '../../src/prelude/format.l',
        '-l', '../../src/prelude/funcall.l',
        '-l', '../../src/prelude/function.l',
        '-l', '../../src/prelude/inspect.l',
        '-l', '../../src/prelude/lambda.l',
        '-l', '../../src/prelude/list.l',
        '-l', '../../src/prelude/loader.l',
        '-l', '../../src/prelude/log.l',
        '-l', '../../src/prelude/macro.l',
        '-l', '../../src/prelude/map.l',
        '-l', '../../src/prelude/namespace.l',
        '-l', '../../src/prelude/parse.l',
        '-l', '../../src/prelude/quasiquote.l',
        '-l', '../../src/prelude/read-macro.l',
        '-l', '../../src/prelude/read.l',
        '-l', '../../src/prelude/repl.l',
        '-l', '../../src/prelude/stream.l',
        '-l', '../../src/prelude/string.l',
        '-l', '../../src/prelude/symbol-macro.l',
        '-l', '../../src/prelude/symbol.l',
        '-l', '../../src/prelude/time.l',
        '-l', '../../src/prelude/type.l',
        '-l', '../../src/prelude/vector.l',
        '-q', '(prelude:%init-ns)'
    ],\
    stdout=subprocess.PIPE,\
    stderr=subprocess.PIPE)
    
    stats = proc.stdout.read()[:-1].decode('utf8')
    err = proc.stderr.read()[:-1].decode('utf8')

    proc.communicate()

    return None if proc.poll == 0 else err

stat_vec = []

for n in range(int(ntests)):
    stat_vec.append(stats()[1:])

print(json.dumps({ 'stats': stat_vec }))
