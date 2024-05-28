import json
import sys
import subprocess

ns = sys.argv[1]
base = sys.argv[2]

with open(base + '/' + ns + '/tests') as f: group_list = f.readlines()

def runtest(line, group, test, expected):
    if ns == 'crux':
        proc = subprocess.Popen(['../dist/mu-sys', '-p', '-e' + test],\
                                stdout=subprocess.PIPE,\
                                stderr=subprocess.PIPE)

    if ns == 'prelude':
        proc = subprocess.Popen(['../dist/mu-sys',
                                 '-l../dist/prelude.l',
                                 '-q (prelude:%init-ns)',
                                 '-p',
                                 '-e' + test],\
                                stdout=subprocess.PIPE,\
                                stderr=subprocess.PIPE)

    obtained = proc.stdout.read()[:-1].decode('utf8')
    err = proc.stderr.read()[:-1].decode('utf-8')

    proc.communicate()

    exception = False if proc.poll() == 0 else True

    if exception:
        print(f'exception: {ns:}/{group}:{line:<5} {err}', file=sys.stderr)

    pass_ = True if obtained == expected else False
    result = { 'expect': expected, 'obtain': obtained }

    return { 'line': line, 'exception': exception, 'pass': pass_, 'result': result }

ns_results = []
for group in group_list:
    results = []
    with open(base + '/' + ns + '/' + group[:-1]) as f: test_source = f.readlines()
    
    line = 0
    for test in test_source:
        line += 1
        fields = test[:-1].split('\t')
        if len(fields) != 2:
            results.append({ 'line': line, 'test syntax': fields })
            continue

        test, expected = fields
        results.append(runtest(line, group[:-1], test, expected))

    ns_results.append({'group': group[:-1], 'results': results})

print(json.dumps({ 'ns': sys.argv[1], 'results': ns_results }))
