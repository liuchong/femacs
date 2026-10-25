#!/usr/bin/env python3
"""Build once through the Rust owner and require its complete structured package receipt."""
import argparse
import json
from pathlib import Path
import subprocess
import sys

ROOT=Path(__file__).resolve().parent.parent


def main(argv=None):
    parser=argparse.ArgumentParser()
    parser.add_argument('--keep',type=Path)
    parser.add_argument('--native',type=Path)
    parser.add_argument('--assets',action='store_true')
    args=parser.parse_args(argv)
    command=['cargo','run','-q','-p','xtask','--','android','package']
    if args.keep:command+=['--out',str(args.keep)]
    if args.native:command+=['--native-so',str(args.native)]
    if args.assets:command+=['--assets']
    result=subprocess.run(command,cwd=ROOT,stdout=subprocess.PIPE,stderr=None)
    if result.returncode:return result.returncode
    try:
        report=json.loads(result.stdout)
        if report.get('schema')!='femacs-android-package-v1' or report.get('status')!='passed' or report.get('errors') or report.get('signer',{}).get('verified') is not True:
            raise ValueError('complete verified package receipt missing')
        print(json.dumps(report,indent=2))
        return 0
    except (ValueError,TypeError) as error:
        print('FAIL '+str(error),file=sys.stderr);return 1


if __name__=='__main__':
    raise SystemExit(main())
