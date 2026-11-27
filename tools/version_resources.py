"""Render the two owned version stamps into an explicit source/staging root."""
from pathlib import Path
import re


def render_versions(root:Path)->None:
    candidate=root/'crates/lisp-vm/src/ported/src/emacs.rs'
    source=(candidate if candidate.is_file() else root/'src/emacs.rs').read_text()
    match=re.search(r'PACKAGE_VERSION:\s*&CStr\s*=\s*c"([^"]+)"',source)
    if match is None:raise ValueError('owned source has no PACKAGE_VERSION')
    version=match[1]
    for template,output,token,value in [
        ('doc/emacs/emacsver.texi.in','doc/emacs/emacsver.texi','@version@',version),
        ('etc/refcards/emacsver.tex.in','etc/refcards/emacsver.tex','@majorversion@',version.split('.')[0]),
    ]:
        destination=root/output;destination.parent.mkdir(parents=True,exist_ok=True)
        destination.write_text((root/template).read_text().replace(token,value))
