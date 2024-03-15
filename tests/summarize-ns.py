import json
import sys
from datetime import datetime

with open(sys.argv[1]) as f: test_results = json.load(f)

print(f'Namespace Test Summary:')
print('-----------------------')

ns = test_results['ns']
groups = test_results['results']

test_total = 0
test_fails = 0
test_exceptions = 0

for group in groups:
    group_label = group['group']
    results = group['results']

    total = 0
    passed = 0
    exceptions = 0
    for result in results:
        total += 1
        passed += 1 if result['pass'] else 0
        exceptions += 1 if result['exception'] else 0

    failed = total - passed - exceptions
    print(f'{ns:<10} {group_label:<14} total: {total:<8} pass: {passed:<8} fail: {failed:<8} exceptions: {exceptions:<8}')
    test_total += total
    test_fails += failed
    test_exceptions += exceptions

test_passes = test_total - (test_fails + test_exceptions)
print('-----------------------')
print(f'{ns:<26}', end='')
print(f'total: {test_total:<9}', end='')
print(f'pass: {test_passes:<9}', end='')
print(f'fail: {test_fails:<9}', end='')
print(f'exceptions: {test_exceptions:<10}')
print()
