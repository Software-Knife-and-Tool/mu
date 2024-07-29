import sys
import os
import re
from datetime import datetime

with open(sys.argv[1]) as f: src = f.read(-1)

def strip_comment(src):
    return re.sub(r';.*\n', '', re.sub(r'\#\|.*\|\#', ' ', src))

def strip_space(src):
    return re.sub(r'\n', '', re.sub(r'\s[\s]+', ' ', src))
    
def fasl(src):
    date = datetime.now().strftime('%m/%d/%Y %H:%M:%S')
    
    print(f';;; fasl: {sys.argv[1]} {date}')
    src = strip_comment(src)
    src = strip_space(src)

    print(src)

fasl(src)
