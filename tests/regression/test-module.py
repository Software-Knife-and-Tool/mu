import json
import sys
import subprocess

module = sys.argv[1]
base = sys.argv[2]

with open(base + '/' + module + '/tests') as f: group_list = f.readlines()

def runtest(line, group, test, expected):
    if module == 'core':
        proc = subprocess.Popen(['../../dist/mu-sys',
                                 '-l../../dist/core.sys',
                                 '-e' + test],             \
                                stdout=subprocess.PIPE,    \
                                stderr=subprocess.PIPE)

    if module == 'common':
        proc = subprocess.Popen(['../../dist/mu-sys',
                                 '-l../../dist/core.sys',
                                 '-l../../dist/common.fasl',
                                 '-e (core:eval \'{})'.format(test),    \
                                 ],                                     \
                                stdout=subprocess.PIPE,                 \
                                stderr=subprocess.PIPE)

    if module == 'format':
        proc = subprocess.Popen(['../../dist/mu-sys',
                                 '-l../../dist/core.sys',
                                 '-l../../dist/format.sys',
                                 '-e (core:eval \'{})'.format(test),    \
                                 ],                                     \
                                stdout=subprocess.PIPE,                 \
                                stderr=subprocess.PIPE)

    if module == 'module':
        proc = subprocess.Popen(['../../dist/mu-sys',
                                 '-l../../dist/core.sys',
                                 '-l../../dist/module.sys',
                                 '-e (core:eval \'{})'.format(test),    \
                                 ],                                     \
                                stdout=subprocess.PIPE,                 \
                                stderr=subprocess.PIPE)

    if module == 'deftype':
        proc = subprocess.Popen(['../../dist/mu-sys',
                                 '-l../../dist/core.sys',
                                 '-l../../dist/deftype.sys',
                                 '-e (core:eval \'{})'.format(test),    \
                                 ],                                     \
                                stdout=subprocess.PIPE,                 \
                                stderr=subprocess.PIPE)

    obtained = proc.stdout.read()[:-1].decode('utf8')
    err = proc.stderr.read()[:-1].decode('utf-8')

    proc.communicate()

    exception = False if proc.poll() == 0 else True

    if exception:
        print(f'exception: {module:}/{group}:{line:<5} {err}', file=sys.stderr)

    pass_ = True if obtained == expected else False
    result = { 'expect': expected, 'obtain': obtained }

    return { 'line': line, 'exception': exception, 'pass': pass_, 'result': result }

module_results = []
for group in group_list:
    results = []
    with open(base + '/' + module + '/' + group[:-1]) as f: test_source = f.readlines()
    
    line = 0
    for test in test_source:
        line += 1
        fields = test[:-1].split('\t')
        if len(fields) != 2:
            results.append({ 'line': line, 'test syntax': fields })
            continue

        test, expected = fields
        results.append(runtest(line, group[:-1], test, expected))

    module_results.append({'group': group[:-1], 'results': results})

print(json.dumps({ 'module': module, 'results': module_results }))
