###  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
###  SPDX-License-Identifier: MIT

import sys
from datetime import datetime

with open(sys.argv[1]) as f: ref = f.readlines()
date = datetime.now().strftime('%m/%d/%Y %H:%M:%S')

print(f'symbol reference: {date:<10}')
print('unbound symbols ------')

def addr_of(byte_vec):
    rev = list(reversed(byte_vec[:-1].split()[1:]))

    hex_le = [f'{int(byte):02x}' for byte in rev]
    return f'{int(''.join(hex_le), 16):8x}'

ref.sort()
for symbol in ref:
    fields = symbol[:-1].split("\t")
    name, type, value, addr = fields
    if name == 'unbound':
        print(f'{value:<35} {type:<10}')
print('----------------------')

for symbol in ref:
    fields = symbol[:-1].split("\t")
    name, type, value, addr = fields

    if name != 'unbound':
        fvalue = (value[:47] + '...') if len(value) > 50 else value
        print(f'{name:<32} {addr_of(addr):<16} {type:<16} {fvalue:<30}')
