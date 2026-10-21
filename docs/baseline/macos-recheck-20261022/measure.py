import os,subprocess,pathlib,tempfile,json,re,statistics,hashlib,time
root=pathlib.Path.cwd(); work=root/'build/recheck-20261022'; evidence=root/'docs/baseline/macos-recheck-20261022';evidence.mkdir(exist_ok=True)
corpus=root/'tools/pgo/eval.el'
sides={'default':(root/'build/port/src/emacs',{}),'reference':(root/'build/emacs-ref/src/emacs',{}),'candidate-off':(work/'candidate/port/src/emacs',{'FEMACS_BC_HOT':'0'}),'candidate-old-tag':(work/'candidate/port/src/emacs',{'FEMACS_BC_HOT':'1','FEMACS_BC_TAG':'0'})}
rows=[]
with tempfile.TemporaryDirectory(prefix='femacs-recheck-') as home:
 env={'HOME':home,'TMPDIR':home,'PATH':os.defpath,'LC_ALL':'C'}
 for rnd in range(1,4):
  order=list(sides) if rnd%2 else list(reversed(sides))
  for side in order:
   binary,extra=sides[side]; t=time.monotonic()
   try:
    r=subprocess.run([str(binary),'--no-init-file','--no-site-file','--no-site-lisp','--batch','-l',str(corpus)],env={**env,**extra},cwd=binary.parent,capture_output=True,text=True,timeout=30)
    out=r.stdout+r.stderr
    data={'round':rnd,'side':side,'rc':r.returncode,'wall_seconds':time.monotonic()-t,'output':out.replace(str(root),'<repo>').replace(home,'<temporary-home>')}
    data['markers']={name:float(value) for prefix,name,value in re.findall(r'^(BENCH|TASK)\s+(\S+)\s+([0-9.eE+-]+)\s+ms\s*$',out,re.M)}
    if r.returncode or len(data['markers'])!=10: data['invalid']=True
   except subprocess.TimeoutExpired:data={'round':rnd,'side':side,'error':'timeout 30s','invalid':True}
   rows.append(data);print(rnd,side,data.get('rc'),data.get('markers'),flush=True)
   (evidence/'performance-raw.json').write_text(json.dumps(rows,indent=2)+'\n')
   if data.get('invalid'):break
  if any(x.get('invalid') for x in rows):break
summary={}
for side in sides:
 subset=[r for r in rows if r['side']==side and not r.get('invalid')]
 if len(subset)==3:summary[side]={case:statistics.median(r['markers'][case] for r in subset) for case in subset[0]['markers']}
(evidence/'performance-summary.json').write_text(json.dumps(summary,indent=2)+'\n')
(evidence/'candidate-startup.json').write_text((work/'logs/startup.json').read_text())
r=subprocess.run(['sh','tools/differential.sh'],capture_output=True,text=True,timeout=40)
(evidence/'differential.txt').write_text(f'exit_code={r.returncode}\n'+r.stdout+r.stderr)
print('DIFFERENTIAL',r.returncode,r.stdout,flush=True)
print('SUMMARY',json.dumps(summary),flush=True)
