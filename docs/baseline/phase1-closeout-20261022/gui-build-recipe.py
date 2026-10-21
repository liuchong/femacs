from pathlib import Path
import subprocess,os,time,json,re,collections
root=Path.cwd();work=root/'build/macos-gui-20261022';logs=work/'logs';logs.mkdir(parents=True,exist_ok=True)
assert not work.resolve().is_relative_to((root/'emacs').resolve())
report={'baseline':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'scope':'Build existing Swift GUI units, activate Rust NS path in isolation; no replacement UI, no original C objects','swift':[]}
def save(): (work/'result.json').write_text(json.dumps(report,indent=2)+'\n')
for src in sorted((root/'femacs/platform/macos').glob('*.swift')):
 log=logs/(src.stem+'.log');out=work/(src.stem+'.o');t=time.monotonic()
 cmd=['xcrun','swiftc','-parse-as-library','-module-name','femacs_'+src.stem,'-c',str(src),'-o',str(out)]
 with log.open('w') as f: result=subprocess.run(cmd,cwd=root,stdout=f,stderr=subprocess.STDOUT)
 text=log.read_text(errors='replace');errors=[x for x in text.splitlines() if ': error:' in x]
 row={'unit':str(src.relative_to(root)),'exit_code':result.returncode,'seconds':round(time.monotonic()-t,3),'error_count':len(errors),'errors':errors[:12]}
 report['swift'].append(row);save();print(src.stem,result.returncode,len(errors),row['seconds'],flush=True)
# Build the existing NS Rust arm in its own tree. This diagnostic does not claim a fully frozen GUI target.
(work/'ns-flags.json').write_text(json.dumps({'features':{'HAVE_NS':'1','HAVE_WINDOW_SYSTEM':'1','NS_IMPL_COCOA':'1'}},indent=2)+'\n')
env={**os.environ,'FEMACS_CONFIG':'../../../build/macos-gui-20261022/ns-flags','FEMACS_BUILD_ROOT':str(work/'rust'),'CARGO_TARGET_DIR':str(work/'cargo')}
t=time.monotonic();log=logs/'rust-ns.log'
with log.open('w') as f: result=subprocess.run(['cargo','run','-q','-p','xtask','--','build-tu','src/frame.c'],cwd=root/'femacs',env=env,stdout=f,stderr=subprocess.STDOUT)
text=log.read_text(errors='replace');report['rust_ns']={'exit_code':result.returncode,'seconds':round(time.monotonic()-t,3),'errors':re.findall(r'error(?:\[[^\]]+\])?:[^\n]+',text)[:25]};save();print('rust_ns',result.returncode,report['rust_ns']['errors'][:4],flush=True)
# Report currently unresolved non-SDK references without pretending terminal objects are a GUI core.
objects=list(work.glob('*.o'));existing=list((root/'build/femacs').glob('*.o'))
def nm_flags(flags,paths):
 if not paths:return set()
 p=subprocess.run(['nm',*flags,*map(str,paths)],capture_output=True,text=True)
 return {l.split()[-1] for l in p.stdout.splitlines() if l.split() and not l.rstrip().endswith(':')}
imports=nm_flags(['-u','-j'],objects);exports=nm_flags(['-g','-U','-j'],objects+existing)
report['unresolved_vs_terminal_objects']=sorted(imports-exports)
report['unresolved_includes_sdk_symbols']=True
for row in report['swift']:row['errors']=[x.replace(str(root),'<repo>') for x in row['errors']]
save();print('unresolved_vs_terminal_objects',len(report['unresolved_vs_terminal_objects']),flush=True)
