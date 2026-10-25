#!/usr/bin/env python3
"""Validate the bounded 53-theorem audit; a lexical gate, not a Lean parser."""
from pathlib import Path
from collections import Counter
import re
import sys

ALLOWED = {'propext', 'Classical.choice', 'Quot.sound'}

def validate(root: Path, log: str):
    files = [root / 'Emergent.lean', root / 'AxiomAudit.lean', *sorted((root / 'Emergent').glob('*.lean'))]
    declared = []
    for path in files:
        text = path.read_text()
        if re.search(r'\b(sorry|admit|unsafe)\b', text):
            raise ValueError(f'Forbidden token in {path.name}')
        if re.search(r'^\s*(?:private\s+)?axiom\s', text, re.M):
            raise ValueError(f'Custom axiom in {path.name}')
        namespaces = re.findall(r'^namespace\s+(\S+)\s*$', text, re.M)
        names = re.findall(r'^theorem\s+(\w+)', text, re.M)
        declared.extend('.'.join(namespaces + [name]) for name in names)
    expected = re.findall(r'^#print axioms (\S+)$', (root / 'AxiomAudit.lean').read_text(), re.M)
    if len(declared) != 53 or len(set(declared)) != 53 or Counter(declared) != Counter(expected):
        raise ValueError('Source declarations and audit inventory must match exactly (53 unique names)')
    rows = re.findall(r"'([^']+)' (depends on axioms: \[[^\]]*\]|does not depend on any axioms)", log)
    if Counter(name for name, _ in rows) != Counter(expected):
        raise ValueError('Missing, duplicate or unexpected audit output')
    constructive = 0
    for name, result in rows:
        if result == 'does not depend on any axioms':
            constructive += 1
        else:
            axioms = {a.strip() for a in result.split('[', 1)[1].rstrip(']').split(',') if a.strip()}
            if not axioms <= ALLOWED:
                raise ValueError(f'{name}: forbidden axioms {axioms - ALLOWED}')
    return {'theorems': len(rows), 'without_axioms': constructive, 'with_foundations': len(rows)-constructive}

if __name__ == '__main__':
    root = Path(__file__).resolve().parents[1]
    print(validate(root, Path(sys.argv[1]).read_text()))
