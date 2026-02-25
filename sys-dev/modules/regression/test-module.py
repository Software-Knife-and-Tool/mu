import json
import sys
import subprocess

mu_sys = sys.argv[1]
core_sys = sys.argv[2]
module = sys.argv[3]
base = sys.argv[4]

with open(base + '/namespaces/' + module + '/tests') as f: group_list = f.readlines()

core_arg = '-l' + core_sys

def runtest(line, group, test, expected):
    match module:
        case 'core':
            proc = subprocess.Popen([mu_sys,
                                     core_arg,
                                     '-e' + test],             \
                                    stdout=subprocess.PIPE,    \
                                    stderr=subprocess.PIPE)

        case 'common':
            proc = subprocess.Popen([mu_sys,
                                     core_arg,
                                     '-l../../dist/common.fasl',
                                     '-e (core:eval \'{})'.format(test),    \
                                     ],                                     \
                                    stdout=subprocess.PIPE,                 \
                                    stderr=subprocess.PIPE)

        case 'module':
            proc = subprocess.Popen([mu_sys,
                                     core_arg,
                                     '-l../../dist/module.sys',
                                     '-e (core:eval \'{})'.format(test),    \
                                     ],                                     \
                                    stdout=subprocess.PIPE,                 \
                                    stderr=subprocess.PIPE)

        case 'deftype':
            proc = subprocess.Popen([mu_sys,
                                     core_arg,
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
    with open(base + '/namespaces/' + module + '/' + group[:-1]) as f: test_source = f.readlines()
    
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
