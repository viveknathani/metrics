### metrics

Get CPU, memory, network and GPU info (NVIDIA based) in Rust.

Sample output below.

```
=> System Information:
System name:             Some("Windows")
System kernel version:   Some("20348")
System OS version:       Some("10 (20348)")
System host name:        Some("image-creator")
Number of CPUs: 16
=> Memory Information:
Total memory: 68715651072 bytes
Used memory : 7199633408 bytes
Total swap  : 1073741824 bytes
Used swap   : 0 bytes
=> CPU Information:
[CPU] Name: "CPU 1", Usage: 5.0078125%, Brand: "           Intel(R) Xeon(R) CPU @ 2.20GHz", Frequency: 2200 MHz, Vendor ID: "GenuineIntel"
[CPU] Name: "CPU 2", Usage: 0.70443726%, Brand: "           Intel(R) Xeon(R) CPU @ 2.20GHz", Frequency: 2200 MHz, Vendor ID: "GenuineIntel"
[CPU] Name: "CPU 3", Usage: 4.5205765%, Brand: "           Intel(R) Xeon(R) CPU @ 2.20GHz", Frequency: 2200 MHz, Vendor ID: "GenuineIntel"
[CPU] Name: "CPU 4", Usage: 1.4291306%, Brand: "           Intel(R) Xeon(R) CPU @ 2.20GHz", Frequency: 2200 MHz, Vendor ID: "GenuineIntel"
[CPU] Name: "CPU 5", Usage: 2.1273727%, Brand: "           Intel(R) Xeon(R) CPU @ 2.20GHz", Frequency: 2200 MHz, Vendor ID: "GenuineIntel"
[CPU] Name: "CPU 6", Usage: 0.91306305%, Brand: "           Intel(R) Xeon(R) CPU @ 2.20GHz", Frequency: 2200 MHz, Vendor ID: "GenuineIntel"
[CPU] Name: "CPU 7", Usage: 2.5088959%, Brand: "           Intel(R) Xeon(R) CPU @ 2.20GHz", Frequency: 2200 MHz, Vendor ID: "GenuineIntel"
[CPU] Name: "CPU 8", Usage: 0.29440308%, Brand: "           Intel(R) Xeon(R) CPU @ 2.20GHz", Frequency: 2200 MHz, Vendor ID: "GenuineIntel"
[CPU] Name: "CPU 9", Usage: 1.1373138%, Brand: "           Intel(R) Xeon(R) CPU @ 2.20GHz", Frequency: 2200 MHz, Vendor ID: "GenuineIntel"
[CPU] Name: "CPU 10", Usage: 0.42731476%, Brand: "           Intel(R) Xeon(R) CPU @ 2.20GHz", Frequency: 2200 MHz, Vendor ID: "GenuineIntel"
[CPU] Name: "CPU 11", Usage: 6.4536285%, Brand: "           Intel(R) Xeon(R) CPU @ 2.20GHz", Frequency: 2200 MHz, Vendor ID: "GenuineIntel"
[CPU] Name: "CPU 12", Usage: 3.2702942%, Brand: "           Intel(R) Xeon(R) CPU @ 2.20GHz", Frequency: 2200 MHz, Vendor ID: "GenuineIntel"
[CPU] Name: "CPU 13", Usage: 1.540741%, Brand: "           Intel(R) Xeon(R) CPU @ 2.20GHz", Frequency: 2200 MHz, Vendor ID: "GenuineIntel"
[CPU] Name: "CPU 14", Usage: 1.3215027%, Brand: "           Intel(R) Xeon(R) CPU @ 2.20GHz", Frequency: 2200 MHz, Vendor ID: "GenuineIntel"
[CPU] Name: "CPU 15", Usage: 5.630348%, Brand: "           Intel(R) Xeon(R) CPU @ 2.20GHz", Frequency: 2200 MHz, Vendor ID: "GenuineIntel"
[CPU] Name: "CPU 16", Usage: 1.3021393%, Brand: "           Intel(R) Xeon(R) CPU @ 2.20GHz", Frequency: 2200 MHz, Vendor ID: "GenuineIntel"
[CPU] Average Usage: 2.41%
[CPU] Uptime: "0 days, 3 hours, 37 minutes"
=> Network Information:
Ethernet 2: 15100 B/s (down) / 141008 B/s (up)
=> GPU Information:
GPU Utilization: 14%
Memory Utilization: 1175 / 23034 MiB
Encoder Utilization: 4%
Decoder Utilization: 0%
=> Top CPU-Consuming Processes:
"MsMpEng.exe": 5.11%
"Code.exe": 2.27%
"System": 1.45%
"Taskmgr.exe": 0.77%
"Code.exe": 0.68%
"rust-analyzer.exe": 0.63%
"steam.exe": 0.48%
"slack.exe": 0.44%
"NVDisplay.Container.exe": 0.38%
```