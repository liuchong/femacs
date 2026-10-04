// femacs: mirror of nt/inc/netinet/in.h @ fd7afced403.
//
// One of the Windows port's four "null version" shims: `<netinet/in.h>` carries no declarations
// of its own on NT, and its single line says that `<sys/socket.h>` has everything.  The frozen
// configuration never reaches it - no `nt/` translation unit is compiled
// (docs/baseline/frozen-build-sources.txt's batch-F section) and no `src/` unit includes a
// `nt/inc/` header - so `xtask layout-check`, which compiles the `src/include` and
// `lib/include` mirrors, does not compile this one; the mirror exists so the port is complete
// and `docs/port-map.md`'s mapping has a target.  In a MinGW build the include path puts
// `nt/inc` first, and `src/process.c`, `lib-src/pop.c`, `lib-src/emacsclient.c` and
// `lib-src/movemail.c` are the units whose `#include <netinet/in.h>` lands here.
//
// The live inventory is 0 subr / 0 function / 0 macro / 0 type / 0 object, so the mirror is
// this note alone, the way `src/include/getpagesize.rs` records an empty unit.
