#!/usr/bin/env python3
"""Fault injection checks: the audit gate must reject corrupted inputs."""
import shutil
import tempfile
from pathlib import Path
from check_audit import validate

root = Path(__file__).resolve().parents[1]
log = (root / 'verification/axiom-audit.log').read_text()
assert validate(root, log)['theorems'] == 101

def rejected(candidate, report):
    try:
        validate(candidate, report)
    except ValueError:
        return
    raise AssertionError('Corruption was incorrectly accepted')

for label, mutate in [
    ('placeholder', lambda s: s.replace('by trivial', 'by sorry', 1)),
    ('custom axiom', lambda s: s + '\naxiom injected : False\n'),
    ('unsafe', lambda s: s + '\nunsafe def injected : Nat := 0\n'),
    ('unlisted theorem', lambda s: s + '\ntheorem injected : True := by trivial\n'),
]:
    with tempfile.TemporaryDirectory() as tmp:
        candidate = Path(tmp) / 'package'
        shutil.copytree(root, candidate, ignore=shutil.ignore_patterns('.lake', 'verification', '__pycache__'))
        target = candidate / 'Collatz/PhiWords.lean'
        target.write_text(mutate(target.read_text()))
        rejected(candidate, log)
    print('PASS: rejected', label)

lines = log.splitlines()
row_index = next(i for i, line in enumerate(lines) if "'Collatz." in line)
rejected(root, '\n'.join(lines[:row_index]+lines[row_index+1:]))
rejected(root, log+'\n'+lines[row_index])
rejected(root, log.replace('propext', 'Injected.axiom', 1))
rejected(root, log.replace('propext', 'sorryAx', 1))
print('PASS: rejected missing/duplicate rows, foreign axiom and sorryAx; clean baseline accepted')
