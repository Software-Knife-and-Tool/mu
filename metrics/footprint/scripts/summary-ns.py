import json
import sys

from statistics import mean

with open(sys.argv[1]) as f: test_results = json.load(f)
ns = test_results['ns']

# [2, 6, 10, 14, 18, 22, 26]
def storage_bytes(hp_info):
    fields = hp_info[:-1].split()

    total = 0
    for i in range(2, 26, 4):
        total += int(fields[i])
    return total

def time_average(times):
    return mean(list(map(int, times)))

results = []
for group in test_results['results']:
    storage_total = 0
    time_total = 0.0
    for test in group['results']:
        if test['storage'] != '':
            storage_total += storage_bytes(test['storage'])
            time_total += time_average(test['times'])
        
    results.append({ 'test': ns + '/' + group['group'],
                     'storage': storage_total,
                     'times': time_total })

print(f'Perf Metrics Summary: {ns}')
print('--------------------')
for test in results:
    test_name, storage, times = test.values()

    print(f'{test_name:<18} bytes: {storage:>8}     usecs: {times:10.2f}')

print()
# print(json.dumps({ 'ns': ns, 'results': results }))
