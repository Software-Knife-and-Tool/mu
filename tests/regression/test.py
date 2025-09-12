import json
import sys
import subprocess

ns = sys.argv[1]
base = sys.argv[2]
test = sys.argv[3]

with open("namespaces/" + base + '/' + ns + '/' + test) as f: test_source = f.readlines()

def runtest(line, test, expected):
    if ns == 'mu':
        proc = subprocess.Popen(['../../dist/mu-sys',
                                 '-e' + test],             \
                                stdout=subprocess.PIPE,    \
                                stderr=subprocess.PIPE)

    if ns == 'core':
        proc = subprocess.Popen(['../../dist/mu-sys',
                                 '-l../../dist/core.sys',
                                 '-e' + test],             \
                                stdout=subprocess.PIPE,    \
                                stderr=subprocess.PIPE)

    if ns == 'common':
        proc = subprocess.Popen(['../../dist/mu-sys',
                                 '-l../../dist/core.sys',
                                 '-q (core:%require "{}" "../../mu/modules")'.format('common'),
                                 '-e (core:eval \'{})'.format(test),    \
                                 ],                                     \
                                stdout=subprocess.PIPE,                 \
                                stderr=subprocess.PIPE)

    obtained = proc.stdout.read()[:-1].decode('utf8')
    err = proc.stderr.read()[:-1].decode('utf-8')

    proc.communicate()

    exception = False if proc.poll() == 0 else True

    if exception:
        print(f'exception: {ns:}/{test}:{line:<5} {err}', file=sys.stderr)
    
    pass_ = True if obtained == expected else False
    result = { 'expect': expected, 'obtain': obtained }

    return { 'line': line, 'exception': exception, 'pass': pass_, 'test': test, 'result': result }

test_results = []
line = 0
for test in test_source:
    line += 1
    fields = test[:-1].split('\t')
    if len(fields) != 2:
        results.append({ 'line': line, 'test syntax': fields })
        continue

    test, expected = fields
    test_results.append(runtest(line, test, expected))

print(json.dumps({ 'ns': sys.argv[1], 'test': sys.argv[3], 'results': test_results }))
