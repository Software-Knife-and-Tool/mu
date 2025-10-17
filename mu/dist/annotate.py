###  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
###  SPDX-License-Identifier: MIT
###
import os
import re
import sys
import subprocess

with open(sys.argv[1]) as f: profile_map = f.readlines()
with open(sys.argv[2]) as f: reference_map = f.readlines()

def build_reference():
    proc = subprocess.Popen([
        lade_cmd,
        'symbols',
        '--reference',
        '--namespace',
        'core'
    ],\
    stdout=subprocess.PIPE,\
    stderr=subprocess.PIPE)

    out = proc.stdout.read()[:-1].decode('utf8')
    err = proc.stderr.read()[:-1].decode('utf8')

    proc.communicate()

    return out if proc.poll() == 0 else err
            
def sortf(ref):
    return int(ref[1], 16)

def reference_format(reference):
    reference_lines = []
    for line in reference:
        reference_lines.append(re.split(' +|#<|\\[|\\]>', line))

    reference_lines.sort(key=sortf)
    return reference_lines

def find_name(tag, ref_map):
    for i in range(0, len(ref_map)):
        tag_int = int(tag, 16)
        ref_int = int(ref_map[i][1], 16)
        if i < len(ref_map) - 1:
            next_ref_int = int(ref_map[i + 1][1], 16)
            if tag_int > ref_int and tag_int < next_ref_int:
                return ref_map[i][0]
    
def annotate(prof_map, ref_map):
    for line in prof_map:
        fn_desc = line.replace('\n', '').split('\t')
        fn = re.split(' +|#<|\\[|\\]>', fn_desc[0])
        if len(fn) == 7 and fn[2] == ':lambda':
            tag = fn[5].split(':')[1]
            name = find_name(tag, ref_map)
            if name == None:
                print(f'{fn_desc}')
            else:
                print(f'{name} {tag}: {fn_desc[1]}')

annotate(profile_map, reference_format(reference_map))
