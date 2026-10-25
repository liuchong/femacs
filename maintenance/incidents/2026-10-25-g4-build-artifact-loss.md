# G4 D17: unexpected loss of owned build artifacts

Observed on 2026-10-25. This is an incident record, not an authorized cleanup
batch or an attribution of the deletion to a person, agent or program. The
actor and exact deletion time are unknown. Original source, the installed
system reference, text logs and receipts are separate from these build copies.

The completed Android6 build and device13 receipts bind the APK/library
identities below. Fresh Mac receipt identities bind both copied app binaries
and dumps. Those historical passes remain historical evidence; an absent
binary cannot be presented as a currently runnable delivered candidate.

| Owned artifact | Previously recorded SHA256 | Current observation |
| --- | --- | --- |
| `build/acceptance/g4-device-13/device-accept/installed.apk` | `9e076e90440ac5770d622fd97283d0ecc132a7e3292636f2ccece25f9fe607b4` | absent |
| `build/acceptance/g4-device-13/device-accept/installed-libemacs.so` | `ae4f820887f6644d565fa041c0825648adecd53dc48d0015dd6adaf79d511832` | absent |
| `build/acceptance/g4-ccl-macos-products-1/source-export/build/android-acceptance/libemacs.so` | `ae4f820887f6644d565fa041c0825648adecd53dc48d0015dd6adaf79d511832` | absent |
| `build/acceptance/g4-ccl-macos-products-1/source-export/build/android-acceptance/package/emacs.apk` | `9e076e90440ac5770d622fd97283d0ecc132a7e3292636f2ccece25f9fe607b4` | absent |
| `build/acceptance/g4-ccl-macos-products-1/source-export/dist/macos/Femacs.app/Contents/MacOS/Femacs` | `388ddcf446bfeb389e8e0a57a884a3f31a4c7aa367a71a00e76491cbc3ab7347` | absent |
| `build/acceptance/g4-ccl-macos-products-1/source-export/dist/macos/Femacs.app/Contents/Resources/libexec/Femacs.pdmp` | `c3206549897afa5df6d4429856b1f1ced5b25f85a827861a159c5525ec00b1c1` | present |
| `build/acceptance/g4-ccl-macos-products-1/relocated/Femacs.app/Contents/MacOS/Femacs` | `388ddcf446bfeb389e8e0a57a884a3f31a4c7aa367a71a00e76491cbc3ab7347` | absent |
| `build/acceptance/g4-ccl-macos-products-1/relocated/Femacs.app/Contents/Resources/libexec/Femacs.pdmp` | `c3206549897afa5df6d4429856b1f1ced5b25f85a827861a159c5525ec00b1c1` | present |

The original Android units list contains 180 entries; the current
object directory has zero object files. Diagnostic preflight rejected a mixed
relink before starting any compiler. Seven missing immutable exported platform
baseline copies were restored only after checking the original Root files
against the accepted input hashes; their deletion cause is also unknown.
That restored data did not restore the missing products/objects.

The root trial app binary remains present. It is the older trial artifact;
this incident does not upgrade it to the new CCL-fixed candidate or constitute
a new native GUI/IME pass. The saved crash logs, source-bound independent
findings, actual disassembly and diagnostic source remain available. No GC
root was skipped, no collector disabled and no acceptance requirement reduced.

The preceding CLN-23 operation enumerated only stopped AVD transient files;
its exact private path ledger remains separate from this observed loss. It
cannot be used as evidence that these absent artifacts were intentionally
retired or remain recoverable at their old paths. No physical-volume recovery
figure or deletion-actor conclusion is inferred from directory listings.

Next: clarify concurrent cleanup/output retention, rebuild the required matching
objects/products from the retained source and preserve diagnostic candidates
in an explicit retained output location. Continue the bounded static-root
investigation before G4 acceptance, independent final adoption and signed
submission. Current runtime A preparation can proceed without these binaries.
