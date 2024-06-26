import sys
from statistics import mean
from datetime import datetime

with open(sys.argv[1]) as f: test_results = f.readlines()
date = datetime.now().strftime('%m/%d/%Y %H:%M:%S')

nsize = 0
ntests = 0
nth_test = 0
ntimes = 0
test_in = ""
delta_bytes = 0
delta_times = 0.0

def report(info_list):
    global nsize
    global nth_test
    global ntimes
    global test_in
    global delta_bytes
    global delta_times

    if len(info_list) == 5:
        test_name = info_list[0]
        then_bytes = int(info_list[1])
        then_time = float(info_list[2])
        bytes = int(info_list[3])

        if then_bytes == 0:
            return

        bytes_ratio = float(bytes) / float(then_bytes)

        b = ' '
        if bytes != then_bytes:
            nsize += 1
            b = '*'

        if test_in == test_name:
            nth_test += 1
        else:
            nth_test = 1
            test_in = test_name

        if b == '*':
            delta_bytes += bytes - then_bytes

        print(f'[{b:<1}] {nth_test:>02d} {test_name:<16} bytes: ({then_bytes}/{bytes}, {bytes - then_bytes}, {bytes_ratio:.2f})')

print(f'Performance Report {date:<10}')
print('------------------------------')

for test in test_results[1:]:
    ntests += 1
    report(test[:-1].split())

print('------------------------------')
print(f'ntests: {ntests:<4} size: {nsize:>6}')
print(f'deltas:      bytes: {delta_bytes:>5}')
