import json
import sys
from datetime import datetime

with open(sys.argv[1]) as f: test_results = json.load(f)

ns = test_results['ns']
test_name = test_results['test']
print(f'Test Summary: {ns}/{test_name}')
print('-----------------------')

test_total = 0
test_fails = 0
test_passes = 0
test_exceptions = 0

for test in test_results['results']:
    test_total += 1
    test_passes += 1 if test['pass'] else 0
    test_exceptions += 1 if test['exception'] else 0
    test_fails += 1 if not test['pass'] and not test['exception'] else 0

    status = 'exception' if test['exception'] and not test['pass'] else 'fail'

    result = test['result']    
    if test['exception'] or not test['pass']:
        line = test['line']
        test = test['test']
        expect = result['expect']
        obtain = result['obtain']
        ftest = (test[:27] + '...') if len(test) > 30 else test
        ferr = (obtain[:12] + '...') if len(obtain) > 15 else obtain
        fexpect = (expect[:12] + '...') if len(expect) > 15 else expect
        print(f'{line:>3} {ftest:<30} {fexpect:<15} {ferr:<15} {status:<8}')

print('-----------------------')
print(f'{ns:<11}', end='')
print(f'passed: {test_passes:<7}', end='')
print(f'total: {test_total:<9}', end='')
print(f'failed: {test_fails:<9}', end='')
print(f'exceptions: {test_exceptions:<10}')
print()
