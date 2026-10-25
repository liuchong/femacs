#!/usr/bin/env python3
"""Read only current product-bound Android receipts; missing evidence is NOT MET."""
import argparse
import importlib.util
import json
from pathlib import Path
import sys


def main(argv=None):
    parser=argparse.ArgumentParser()
    parser.add_argument('--receipt',type=Path)
    parser.add_argument('--apk',type=Path)
    parser.add_argument('--so',type=Path)
    parser.add_argument('--require',action='store_true')
    args=parser.parse_args(argv)
    result={'passed':False,'errors':['current --receipt, --apk and --so are required']}
    if all((args.receipt,args.apk,args.so)):
        try:
            spec=importlib.util.spec_from_file_location('femacs_android_readiness_device',Path(__file__).with_name('android-device.py'))
            device=importlib.util.module_from_spec(spec);sys.modules[spec.name]=device;spec.loader.exec_module(device)
            result=device.judge(device.load_receipt(args.receipt),args.apk,args.so)
        except Exception as error:result={'passed':False,'errors':[str(error)]}
    print(('MET' if result['passed'] else 'NOT MET')+': '+json.dumps(result,ensure_ascii=False))
    return 0 if result['passed'] or not args.require else 1


if __name__=='__main__':
    raise SystemExit(main())
