"""CLI: sso verify | sso pilot."""

import argparse
import json
import sys

from . import AffineCycle, fingerprints
from .experiment import run_pilot


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(prog="sso")
    sub = parser.add_subparsers(dest="command", required=True)
    v = sub.add_parser("verify")
    v.add_argument("--modulus", type=int, required=True)
    v.add_argument("--multiplier", type=int, required=True)
    v.add_argument("--increment", type=int, required=True)
    v.add_argument("--states", required=True, help="comma-separated orbit")
    p = sub.add_parser("pilot")
    p.add_argument("--max-modulus", type=int, default=8)
    p.add_argument("--threshold", type=int, default=3)
    p.add_argument("--seed", type=int, default=42)
    args = parser.parse_args(argv)
    try:
        if args.command == "verify":
            states = tuple(int(s.strip()) for s in args.states.split(","))
            result = fingerprints(AffineCycle(
                args.modulus, args.multiplier, args.increment, states))
        else:
            result = run_pilot(args.max_modulus, args.threshold, args.seed)
    except ValueError as exc:
        print(json.dumps({"accepted": False, "error": str(exc)}), file=sys.stderr)
        return 2
    print(json.dumps(result, sort_keys=True, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
