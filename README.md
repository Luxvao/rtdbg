# Rtdbg - A real time binary introspection toolkit
Rtdbg aims to be a frida-like tool (it's nowhere close yet and likely never will be) that replaces javascript with rhai and is specifically targeting linux as its main platform. Other platforms might work, but no compatibility is guaranteed and or supported in any way by me. This could change in the future.

## Features
- Runtime injection through LD_PRELOAD
- Arbitrary script injection without process restarts through the socket
- Simple GUI companion app for injecting and running scripts
- A small but growing API exposed to rhai

## Planned Features
- Dynamic function hooking through GOT and PLT
- Process information exposed through the rhai api
- Ptrace-based runtime injection
- Static binary patching to automatically load the runtime without any rtdbg loaders
- Event support in rhai
- Better script managing

## API Features
- Write mem (both strings and vectors as input parameters in rhai)
- Read mem
- `mprotect` access within rhai
