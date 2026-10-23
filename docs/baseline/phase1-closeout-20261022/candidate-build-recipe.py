import os,sys,subprocess,shutil,hashlib
from pathlib import Path
root=Path.cwd(); work=root/"build/recheck-20261022"; candidate=work/"candidate"
units=candidate/"femacs"; units.mkdir(parents=True,exist_ok=True)
for src in (root/"build/femacs").glob("*.o"):
 if not src.name.startswith("bytecode."):
  dst=units/src.name
  if not dst.exists():dst.symlink_to(src)
src=root/"src/bytecode.rs"
original=src.read_bytes()
helper=root/"src/bytecode_hot.rs"
assert not helper.exists()
source=original.decode()
assert 'include!("bytecode_hot.rs");' not in source
source=source.replace("BYTE_CODES! ();","BYTE_CODES! ();\ninclude!(\"bytecode_hot.rs\");",1)
needle="      op = FETCH! ();"
assert source.count(needle)==1
bridge="""      #[cfg(all(target_arch = "aarch64", target_os = "macos", not(enable_checking)))]
      if bc_hot_enabled () {
        let mut hot = BcHotState {
          top, pc, vectorp, bytestr_data, bytestr_length, const_length,
          quitcounter, _pad: [0;7], qt: Qt(), qnil: Qnil(), qunbound: Qunbound(),
          lispsym: &raw mut lispsym as *mut Lisp_Symbol, fix_tag: bc_hot_tag()
        };
        let _ = bc_hot_run(&mut hot);
        top = hot.top;
        pc = hot.pc;
      }

"""
source=source.replace(needle,bridge+needle,1)
helper.write_bytes((work/"candidate-hot.rs").read_bytes())
env=os.environ.copy()
env.update(FEMACS_BUILD_ROOT=str(candidate),CARGO_TARGET_DIR=str(work/"cargo"),FEMACS_PGO="off")
env.pop("FEMACS_TARGET",None)
try:
 src.write_text(source)
 command='cd femacs && cargo run -q -p xtask -- build-tu src/bytecode.c && cargo run -q -p xtask -- gate-load && cargo run -q -p xtask -- dump'
 with (work/"logs/candidate-build.log").open("w") as log:
  result=subprocess.run(["python3",str(Path.home()/".assistant/skills/controlled-run/scripts/run.py"),"--profile","build","--","sh","-c",command],env=env,stdout=log,stderr=subprocess.STDOUT)
 print("candidate build exit",result.returncode)
 sys.exit(result.returncode)
finally:
 src.write_bytes(original)
 helper.unlink()
 assert hashlib.sha256(src.read_bytes()).digest()==hashlib.sha256(original).digest()
