###  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
###  SPDX-License-Identifier: MIT
###
import os
import sys
import subprocess

mu_cmd = '/opt/mu/bin/mu-sys'

src = sys.argv[1]

def fasl():
    proc = subprocess.Popen([
        mu_cmd,
        '-p',
        '-l', '/opt/mu/dist/core.l',
        '-q', '(prelude:%init-ns)',
        '-l', './fasl.l',
        '-q', f'(fasl:fasl "{src}" :t)'
    ],\
    stdout=subprocess.PIPE,\
    stderr=subprocess.PIPE)
    
    vector = proc.stdout.read()[:-1].decode('utf8')
    err = proc.stderr.read()[:-1].decode('utf8')

    proc.communicate()

    return vector if proc.poll == 0 else err

print(fasl())
