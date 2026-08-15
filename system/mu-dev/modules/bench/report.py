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
delta_mem  = 0

def report(info_list):
    global nsize
    global nth_test
    global ntimes
    global test_in
    global delta_bytes
    global delta_times
    global delta_mem

    if len(info_list) == 7:        
        test_name = info_list[0]
        base_bytes = int(info_list[1])
        base_time = float(info_list[2])
        base_mem = int(info_list[3])
        bytes = int(info_list[4])
        time = float(info_list[5])
        mem = int(info_list[6])

        if base_bytes == 0:
            return

        base_pages = int((base_mem + 4095) / 4096)
        mem_pages = int((mem + 4095) / 4096)

        mem_ratio =  float(mem_pages) / float(base_pages)
        bytes_ratio = float(bytes) / float(base_bytes)
        time_ratio = time / base_time

        b = ' '
        if bytes != base_bytes:
            nsize += 1
            if bytes < 0:
                b = '*'
            elif bytes < base_bytes:
                b = '-'
            else:
                b = '+'

        t = ' '
        if time < 0.0:
            t = '*'
            ntimes += 1
        elif time_ratio > 1 + .15 or time_ratio < 1 - .15:
            ntimes += 1
            if time < base_time:
                t = '-'
            else:
                t = '+'

        m = ' '
        if mem != base_mem:
            if mem < base_mem:
                m = '-'
            else:
                mu = '+'

        if test_in == test_name:
            nth_test += 1
        else:
            nth_test = 1
            test_in = test_name

        if b != ' ' or t != ' ':
            time_diff = time - base_time 
            delta_bytes += bytes - base_bytes
            delta_times += time_diff
            delta_mem += mem - base_mem
            print(f'[{b:<1}{m:<1}{t:<1}] {nth_test:>02d} {test_name:<16} heap: ({base_bytes}/{bytes}, {bytes - base_bytes}, {bytes_ratio:.2f}) \t pages: ({base_pages}/{mem_pages}, {mem_pages - base_pages}, {mem_ratio:.2f}) \ttimes: ({base_time:.2f}/{time:.2f}, {time_diff:.2f}, {time_ratio:.2f})')

print(f'Performance Report {date:<10}')
print('------------------------------')

for test in test_results[1:]:
    ntests += 1
    report(test[:-1].split())

print('------------------------------')
print(f'ntests: {ntests:<4} size: {nsize:>6}  times: {ntimes:>5}')
print(f'deltas:      bytes: {delta_bytes:>5}  mem_virt: {delta_mem:>5}  times: {delta_times:5.2f}')
