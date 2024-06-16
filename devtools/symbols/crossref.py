###  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
###  SPDX-License-Identifier: MIT

import sys
from datetime import datetime

with open(sys.argv[1]) as f: xref = f.readlines()
date = datetime.now().strftime('%m/%d/%Y %H:%M:%S')

print(f'cross reference: {date:<10}')
print('----------------------')

line_no = 0
xref.sort()

for symbol in xref:
    fields = symbol[:-1].split("\t")
    name, type, value = fields
    if name == 'unbound':
        line_no += 1        
        print(f'{line_no:<5} {value:<35} {type:<10}')
print('----------------------')

line_no = 0
for symbol in xref:
    fields = symbol[:-1].split("\t")
    name, type, value = fields

    if name != 'unbound':
        line_no += 1
        fvalue = (value[:47] + '...') if len(value) > 50 else value
        print(f'{line_no:04d} {name:<35} {type:<10} {fvalue:<30}')

print()
print(f'reverse cross reference: {date:<10}')
print('----------------------')

line_no = 0
revxref = []

for symbol in xref:
    fields = symbol[:-1].split("\t")
    name, type, value = fields

    if name != 'unbound' and type == 'func':
        tag = value[:-2].split()
        a, b, r, t = tag
        addr = t[4:].rjust(16, ' ')

        scope = 'extern'
        if name[0] == ':':
            name = name[1:]
            scope = 'intern'

        revxref.append((addr, type, scope, name))

revxref.sort()
for symbol in revxref:
    line_no += 1
    addr, type, scope, name = symbol;

    print(f'{line_no:04d} {addr:<16}    {type}  {scope}  {name:<30}')
