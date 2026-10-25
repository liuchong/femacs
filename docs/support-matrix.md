# Support obligations and validation layers

G4 construction scope is defined in [tooling-modernization.md](tooling-modernization.md). This matrix records retained obligations, not unearned certificates. A/B/C are written; consolidated D has real Mac build/Info and Android native/package evidence, with device/final review/delivery still pending.

| Surface | Retained obligation | Evidence boundary / present gap |
| --- | --- | --- |
| macOS arm64 terminal | Independent source generation, build/dump, classic editing keys, files and Info | Existing delivered baseline has real local evidence. G4 fresh affected build and exact default Info lookup passed; remaining native GUI cases stay separate. |
| macOS arm64 Cocoa | Rust/Swift native app, signed complete resources, relocatable package, classic input/IME/files | Existing baseline is runnable. Actual external Finder drag and multi-window IME remain unaccepted while desktop access is unavailable; batch/packaging cannot close them. |
| Android arm64 | Owned Rust/Kotlin/config/source inputs, normal/checking native links, signed package and current owned emulator acceptance | Existing manifest min API21 and target37 remain. Host SDK and selected emulator API36 are different facts. G4 current native/checking gates and signed resource-bearing package passed their actual scope; owned API36 device acceptance is in progress. |
| Android API21/22 compatibility | Keep the existing minimum; use only floor-or-newer branches | Source floor proof, three extracted-method JVM fixtures across21/22/23/24/26/30/36 and actual Kotlin/dex compile are separate from real API21 device certification. That device certification is not claimed. |
| Android Java/Kotlin | Current latest LTS JDK25 at this dated stage; complete owned Kotlin closure and stdlib in dex | `--release17` is target bytecode, not permission to install an old JDK. Missing current tools fail visibly. No ambient/JBR downgrade. |
| Android other ABIs; Linux; Windows; Haiku; other retained targets | retained/deferred source and applicable raw inputs | No platform was retired because it lacks current-machine tests. A future support retirement needs its exact scope and impact. |
| Raw Lisp/data/manuals/resources | Preserve owned originals and generation inputs | Generated files are rebuilt from those inputs. Required Python/AWK generators, Swift/Kotlin platform code and compiler audit fixtures stay. |
| Performance/native/collector candidates | G5 measurements precede G6/G7 source-bound designs and real VM candidates | G4 does not introduce a VM, dispatch, JIT or collector algorithm change. Old negative experiments remain evidence. |

Android device modes create a fresh run-owned AVD, adb server and explicit serial. Inspection/dump requires an explicit receiver and is read-only. No user AVD replacement, global server reset, root, uninstall or shared diagnostic deletion is part of the new lifecycle. A structurally valid signed APK is not device acceptance; a diagnostic or old image is not readiness.
