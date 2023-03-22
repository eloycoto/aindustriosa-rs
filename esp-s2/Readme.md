# ESP-32 S2 project init


# Dependencies:

```
cargo install espflash@"2.0.0-rc.3"
```


## Running

```
$ --> cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running `espflash flash --monitor target/xtensa-esp32s2-none-elf/debug/esp_s2`
[2023-03-23T08:14:01Z INFO ] Detected 3 serial ports
[2023-03-23T08:14:01Z INFO ] Ports which match a known common dev board are highlighted
[2023-03-23T08:14:01Z INFO ] Please select a port
[2023-03-23T08:14:03Z INFO ] Serial port: '/dev/ttyUSB0'
[2023-03-23T08:14:03Z INFO ] Connecting...
[2023-03-23T08:14:04Z INFO ] Using flash stub
Chip type:         esp32s2 (revision v0.0)
Crystal frequency: 40MHz
Flash size:        4MB
Features:          WiFi, No Embedded Flash, No Embedded PSRAM, ADC and temperature sensor calibration in BLK2 of efuse V2
MAC address:       58:cf:79:38:a7:8a
App/part. size:    148,528/4,128,768 bytes, 3.60%
[00:00:01] [========================================]      14/14      0x1000
[00:00:00] [========================================]       1/1       0x8000
[00:00:05] [========================================]      54/54      0x10000                                                                                                                                                                                                                                                                       [2023-03-23T08:14:11Z INFO ] Flashing has completed!
Commands:
    CTRL+R    Reset chip
    CTRL+C    Exit

ESP-ROM:esp32s2-rc4-20191025
Build:Oct 25 2019
rst:0x1 (POWERON),boot:0x8 (SPI_FAST_FLASH_BOOT)
SPIWP:0xee
mode:DIO, clock div:1
load:0x3ffe6108,len:0x17a8
0x3ffe6108 - _heap_start
    at ??:??
load:0x4004c000,len:0xaa4
0x4004c000 - _text_heap_start
    at ??:??
load:0x40050000,len:0x3138
0x40050000 - _text_heap_end
    at ??:??
SHA-256 comparison failed:
Calculated: bcd79ffcd484f3d79f55295b5fcda5bf870bc7266bdf94fb457ce980f388aa51
Expected: c6a9f1f51dcb9bb64da67bb52670e9f8268e705f7715784bf2f9d4ae572324ef
Attempting to boot anyway...
entry 0x4004c1bc
0x4004c1bc - _text_heap_start
    at ??:??
I (40) boot: ESP-IDF v5.0-beta1-764-gdbcf640261 2nd stage bootloader
I (40) boot: compile time 11:31:32
I (40) boot: chip revision: V000
I (44) boot.esp32s2: SPI Speed      : 80MHz
I (49) boot.esp32s2: SPI Mode       : DIO
I (54) boot.esp32s2: SPI Flash Size : 4MB
I (58) boot: Enabling RNG early entropy source...
I (64) boot: Partition Table:
I (67) boot: ## Label            Usage          Type ST Offset   Length
I (75) boot:  0 nvs              WiFi data        01 02 00009000 00006000
I (82) boot:  1 phy_init         RF data          01 01 0000f000 00001000
I (89) boot:  2 factory          factory app      00 00 00010000 003f0000
I (97) boot: End of partition table
I (101) esp_image: segment 0: paddr=00010020 vaddr=3f000020 size=06174h ( 24948) map
I (115) esp_image: segment 1: paddr=0001619c vaddr=40022000 size=01638h (  5688) load
I (120) esp_image: segment 2: paddr=000177dc vaddr=00000000 size=0883ch ( 34876)
I (133) esp_image: segment 3: paddr=00020020 vaddr=40080020 size=143e8h ( 82920) map
I (152) boot: Loaded app from partition at offset 0x10000
I (152) boot: Disabling RNG early entropy source...
Hello world!
```

