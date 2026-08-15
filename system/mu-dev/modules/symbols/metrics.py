###  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
###  SPDX-License-Identifier: MIT
import re
import sys
from datetime import datetime
from operator import *

with open(sys.argv[1]) as f: prelude = f.readlines()
with open(sys.argv[2]) as f: ns_syms = f.readlines()

date = datetime.now().strftime('%m/%d/%Y %H:%M:%S')

symbols = []
def symbols_in(line):
    if len(line) == 0 or line[0] == ';':
        return
    
    tokens = re.split(r"[ ,\~\#\.\`\'\@;\(\)]", line)
    if len(tokens) == 0:
        return

    for token in tokens:
        if len(token) != 0 and token[0] != '"' and token[0] != ':':
            symbols.append(token)

for line in prelude:
    symbols_in(line[:-1])

counts = []
for symbol in set(symbols):
    nof = countOf(symbols, symbol)
    counts.append([symbol, nof])

ns_syms.sort()

def map_symbol(symbol):
    for count in counts:
        if sym == count[0]:
            return count[1]
    return None

print()
print(f'symbol counts: {date:<10}')
print('-------------------')

by_count = []
for ns_sym in ns_syms:
    sym = ns_sym[:-1]
    count = map_symbol(sym)
    if count != None:
        by_count.append([count, sym])

by_count.sort(reverse=True)
for count in by_count:
    sym = count[1]
    nof = count[0]
    print(f'{sym:<30} {nof}')
