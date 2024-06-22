###  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
###  SPDX-License-Identifier: MIT
###
import json
import os
import sys
import subprocess

ntests = sys.argv[1]

mu_cmd = '../../dist/mu-sys'
time_cmd = 'time'
format = '"%S %U %e %M %w %Z"'

def stats():
    proc = subprocess.Popen([
        time_cmd,
        '-f',
        format,
        mu_cmd,
        '-p',
        '-l', '../../lib/prelude/prelude.l',
        '-l', '../../lib/prelude/break.l',
        '-l', '../../lib/prelude/compile.l',
        '-l', '../../lib/prelude/describe.l',
        '-l', '../../lib/prelude/environment.l',
        '-l', '../../lib/prelude/exception.l',
        '-l', '../../lib/prelude/fixnum.l',
        '-l', '../../lib/prelude/format.l',
        '-l', '../../lib/prelude/funcall.l',
        '-l', '../../lib/prelude/function.l',
        '-l', '../../lib/prelude/inspect.l',
        '-l', '../../lib/prelude/lambda.l',
        '-l', '../../lib/prelude/list.l',
        '-l', '../../lib/prelude/loader.l',
        '-l', '../../lib/prelude/log.l',
        '-l', '../../lib/prelude/macro.l',
        '-l', '../../lib/prelude/map.l',
        '-l', '../../lib/prelude/namespace.l',
        '-l', '../../lib/prelude/parse.l',
        '-l', '../../lib/prelude/quasiquote.l',
        '-l', '../../lib/prelude/read-macro.l',
        '-l', '../../lib/prelude/read.l',
        '-l', '../../lib/prelude/repl.l',
        '-l', '../../lib/prelude/stream.l',
        '-l', '../../lib/prelude/string.l',
        '-l', '../../lib/prelude/symbol-macro.l',
        '-l', '../../lib/prelude/symbol.l',
        '-l', '../../lib/prelude/time.l',
        '-l', '../../lib/prelude/type.l',
        '-l', '../../lib/prelude/vector.l',
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
