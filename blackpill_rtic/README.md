# STM32 quickstart

An example project using STM32F410 Nucleo devices.


## How to run:

Run openocd to connect to device's JTAG until it opens 127.0.0.1:3333

```
sudo openocd -f openocd.cfg
```

In another terminal, you can compile and attach a gdb instance to it:

```
$ --> cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `gdb -q -x openocd.gdb target/thumbv7m-none-eabi/debug/stm32`
Reading symbols from target/thumbv7m-none-eabi/debug/stm32...
warning: Remote gdbserver does not support determining executable automatically.
RHEL <=6.8 and <=7.2 versions of gdbserver do not support such automatic executable detection.
The following versions of gdbserver support it:
- Upstream version of gdbserver (unsupported) 7.10 or later
- Red Hat Developer Toolset (DTS) version of gdbserver from DTS 4.0 or later (only on x86_64)
- RHEL-7.3 versions of gdbserver (on any architecture)
vcell::VolatileCell<u32>::get<u32> (self=0xe0001004) at /home/eloy/.cargo/registry/src/github.com-1ecc6299db9ec823/vcell-0.1.3/src/lib.rs:33
33              unsafe { ptr::read_volatile(self.value.get()) }
Breakpoint 1 at 0x8008116: file src/lib.rs, line 1053.
Note: automatically using hardware breakpoints for read-only addresses.
Breakpoint 2 at 0x800975c: file src/lib.rs, line 1046.
Breakpoint 3 at 0x8009172: file src/lib.rs, line 32.
Breakpoint 4 at 0x8000cb2: file src/main.rs, line 11.
semihosting is enabled
Loading section .vector_table, size 0x1c8 lma 0x8000000
Loading section .text, size 0x9598 lma 0x80001c8
Loading section .rodata, size 0x1648 lma 0x8009760
Start address 0x080001c8, load size 44456
Transfer rate: 23 KB/sec, 8891 bytes/write.
halted: PC: 0x0800811a
cortex_m_rt::DefaultPreInit () at src/lib.rs:1058
1058    pub unsafe extern "C" fn DefaultPreInit() {}
(gdb)
```
