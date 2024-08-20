import json
import sys

from statistics import mean

with open(sys.argv[1]) as f: test_results = json.load(f)

stats = test_results['stats']

system_secs = 0.0
user_secs = 0.0
elapsed_time = 0
resident_size = 0
waits = 0

ntests = len(stats)

for test in stats:
    values = test[:-1].split()

    system_secs += float(values[0])
    user_secs += float(values[1]) 
    elapsed_time += float(values[2])
    resident_size += int(values[3])
    waits += int(values[4])

print(f'system   {system_secs / float(ntests):4.2f}')
print(f'user     {user_secs / float(ntests):4.2f}')
print(f'elapsed  {user_secs / float(ntests):4.2f}')
print(f'resident {resident_size / float(ntests):4.2f}')
print(f'waits    {waits / float(ntests):4.2f}')
