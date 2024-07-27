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
        '-l', '../../dist/prelude/core.l',
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
