###  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
###  SPDX-License-Identifier: MIT
###
import json
import os
import sys
import subprocess

mu_sys = sys.argv[1]
core = sys.argv[2]
module = sys.argv[3]
ns = sys.argv[4]
tests = sys.argv[5]
ntests = sys.argv[6]

perf_arg = '-l' + os.path.join(module, 'perf.l')
core_arg = '-l' + core

with open(os.path.join(tests, 'namespaces', ns, 'tests')) as f: perf_groups = f.readlines()

def mem_virt(ns, group, line, test):
    match ns:
        case 'mu':
            proc = subprocess.Popen([mu_sys,
                                     perf_arg,
                                     '-e (perf:mem-delta (:lambda ()' + test + ') :nil)'],\
                                    stdout=subprocess.PIPE,\
                                    stderr=subprocess.PIPE)

        case 'frequent':
            proc = subprocess.Popen([mu_sys,
                                     perf_arg,
                                     '-e (perf:mem-delta (:lambda ()' + test + ') :nil)'],\
                                    stdout=subprocess.PIPE,\
                                    stderr=subprocess.PIPE)

        case 'format':
            proc = subprocess.Popen([mu_sys,
                                     perf_arg,
                                     '-l./dist/format.sys',
                                     '-e (perf:mem-delta (:lambda ()' + test + ') :nil)'],\
                                    stdout=subprocess.PIPE,\
                                    stderr=subprocess.PIPE)

        case 'core':
            proc = subprocess.Popen([mu_sys,
                                     core_arg,
                                     perf_arg,
                                     '-e (perf:mem-delta (:lambda () {})'.format(test) + ' :nil)'],\
                                    stdout=subprocess.PIPE,\
                                    stderr=subprocess.PIPE)

        case 'common':
            proc = subprocess.Popen([mu_sys,
                                     core_arg,
                                     perf_arg,
                                     '-q (core:%require "{}" "../../mu/modules")'.format('common'),
                                     '-e (perf:mem-delta (:lambda () (core:eval \'{})'.format(test) + ') :nil)'],\
                                    stdout=subprocess.PIPE,                 \
                                    stderr=subprocess.PIPE)

    mem_virt = proc.stdout.read()[:-1].decode('utf8')
    err = proc.stderr.read()[:-1].decode('utf8')

    proc.communicate()

    if len(err) != 0:
        print(f'exception: {ns}/{group}:{line:<5} {err}', file=sys.stderr)
    
    return None if len(err) != 0 else mem_virt

def storage(ns, group, line, test):
    match ns:
        case 'mu':
            proc = subprocess.Popen([mu_sys,
                                     perf_arg,
                                     '-e (perf:storage-delta (:lambda ()' + test + ') :nil)'],\
                                    stdout=subprocess.PIPE,\
                                    stderr=subprocess.PIPE)

        case 'frequent':
            proc = subprocess.Popen([mu_sys,
                                     perf_arg,
                                     '-e (perf:storage-delta (:lambda ()' + test + ') :nil)'],\
                                    stdout=subprocess.PIPE,\
                                    stderr=subprocess.PIPE)

        case 'format':
            proc = subprocess.Popen([mu_sys,
                                     perf_arg,
                                     '-l./dist/format.sys',
                                     '-e (perf:storage-delta (:lambda ()' + test + ') :nil)'],\
                                    stdout=subprocess.PIPE,\
                                    stderr=subprocess.PIPE)

        case 'core':
            proc = subprocess.Popen([mu_sys,
                                     core_arg,
                                     perf_arg,
                                     '-e (perf:storage-delta (:lambda () {})'.format(test) + ' :nil)'],\
                                    stdout=subprocess.PIPE,\
                                    stderr=subprocess.PIPE)

        case 'common':
            proc = subprocess.Popen([mu_sys,
                                     core_arg,
                                     perf_arg,
                                     '-q (core:%require "{}" "../../mu/modules")'.format('common'),
                                     '-e (perf:storage-delta (:lambda () (core:eval \'{})'.format(test) + ') :nil)'],\
                                    stdout=subprocess.PIPE,                 \
                                    stderr=subprocess.PIPE)

    storage_ = proc.stdout.read()[:-1].decode('utf8')
    err = proc.stderr.read()[:-1].decode('utf8')

    proc.communicate()

    if len(err) != 0:
        print(f'exception: {ns}/{group}:{line:<5} {err}', file=sys.stderr)
    
    return None if len(err) != 0 else storage_

def timing(ns, test):
    match ns:
        case 'mu':
            proc = subprocess.Popen([mu_sys,
                                     perf_arg,
                                     '-e (perf:time-delta (:lambda () ' + test + ') :nil)'],\
                                    stdout=subprocess.PIPE,\
                                    stderr=subprocess.PIPE)

        case 'frequent':
            proc = subprocess.Popen([mu_sys,
                                     perf_arg,
                                     '-e (perf:time-delta (:lambda () ' + test + ') :nil)'],\
                                    stdout=subprocess.PIPE,\
                                    stderr=subprocess.PIPE)

        case 'format':
            proc = subprocess.Popen([mu_sys,
                                     perf_arg,
                                     '-l./dist/format.sys',
                                     '-e (perf:time-delta (:lambda () ' + test + ') :nil)'],\
                                    stdout=subprocess.PIPE,\
                                    stderr=subprocess.PIPE)

        case 'core':
            proc = subprocess.Popen([mu_sys,
                                     core_arg,
                                     perf_arg,
                                     '-e (perf:time-delta (:lambda () {})'.format(test) + ' :nil)'],\
                                    stdout=subprocess.PIPE,\
                                    stderr=subprocess.PIPE)

        case 'common':
            proc = subprocess.Popen([mu_sys,
                                     core_arg,
                                     perf_arg,
                                     '-q (core:%require "{}" "../../mu/modules")'.format('common'),
                                     '-e (perf:time-delta (:lambda () (core:eval \'{})'.format(test) + ') :nil)'],\
                                    stdout=subprocess.PIPE,\
                                    stderr=subprocess.PIPE)
    
    time = proc.stdout.read()[:-1].decode('utf8')
    err = proc.stderr.read()[:-1].decode('utf8')

    proc.communicate()

    return None if proc.poll == 0 else time

ns_results = []
for group in perf_groups:
    with open(os.path.join(tests, 'namespaces', ns, group[:-1])) as f: group_source = f.readlines()
    
    storage_ = None
    results = []

    line = 0
    for test in group_source:
        line += 1
        storage_ = storage(ns, group[:-1], line, test[:-1])

        if storage_ == None:
            break

        times = []
        for n in range(int(ntests)):
            times.append(timing(ns, test[:-1]))

        mem_virt_ = mem_virt(ns, group[:-1], line, test[:-1])
        results.append({ 'line': line, 'storage': storage_, 'times': times, 'mem_virt': mem_virt_ })

    ns_results.append({'group': group[:-1], 'results': results })

print(json.dumps({ 'ns': ns, 'results': ns_results }))
