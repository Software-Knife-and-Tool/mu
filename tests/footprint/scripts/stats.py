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
format = '"%S %U %e %M %w %Z'

def times():
    proc = subprocess.Popen([
        time_cmd,
        '-f',
        format,
        mu_cmd,
        '-l', '../../dist/core.fasl',
    ],\
    stdout=subprocess.PIPE,\
    stderr=subprocess.PIPE)
    
    stats = proc.stdout.read()[:-1].decode('utf8')
    err = proc.stderr.read()[:-1].decode('utf8')

    proc.communicate()

    return stats if proc.poll == 0 else err

def storage():
    proc = subprocess.Popen([
        mu_cmd,
        '-l', '../../dist/core.fasl',
        '-e', '(env:heap-stat)'
    ],\
    stdout=subprocess.PIPE,\
    stderr=subprocess.PIPE)
    
    heap = proc.stdout.read()[:-1].decode('utf8')
    err = proc.stderr.read()[:-1].decode('utf8')
    
    proc.communicate()

    heap_vec = heap.replace(')', '').split()
    heaps = []

    heaps.append(heap_vec[5:9])
    heaps.append(heap_vec[9:13])
    heaps.append(heap_vec[13:17])
    heaps.append(heap_vec[17:21])
    heaps.append(heap_vec[21:25])
    heaps.append(heap_vec[25:29])

    return heaps if proc.poll() == 0 else err

stats_vec = []
for n in range(int(ntests)):
    stats_vec.append(times()[1:])

print(json.dumps({ 'room': storage()[1:], 'stats': stats_vec }))
