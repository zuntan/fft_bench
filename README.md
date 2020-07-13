# fft_bench

```
$ git clone https://github.com/zuntan/fft_bench.git
$ cd fft_bench
$ cargo check
$ ( TIME="\nTIME R:%e S:%S U:%U P:%P CMD:%C"; \time bash run_bench.sh 2>&1 ) | tee result/result.txt
```

# About this benchmark program

This program internally generates a sine wave and FFT analyzes the data.

Then, the benchmark is obtained by measuring the compilation time of the program and the processing time of the program.

This program has the following options.


```
$ cargo run -- --help
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/fft_bench --help`
Usage: target/debug/fft_bench [options]

Options:
    -w, --wav [tmp.wav] wav file output
    -x, --fftbw_wav [tmp_fftbw.wav]
                        wav file output (fft backword)
    -l, --len 15        wav file output len (sec)
    -f, --freq 1000     frequency (20-20000)
    -6, --f64           use f64 insteed of f32
    -s, --skip_fft      skip fft
    -b, --enable_fft_backward
                        enable fft_backward
    -v, --version       Print version info and exit.
        --help          Print this help menu.
```

 - -l
    - The time of the wave data to generate. The unit is seconds.

 - -f
    - The frequency of the generated sine wave. You can specify more than one. The default is the A chord.

 - -6
    - The type of FFT processing is f64. The default is f32.

 - -w
    - Outputs the generated data with the file name specified by wave.

 - -s
    - FFT processing is skipped.

 - -b
    - Performs inverse FFT processing.

 - -x
    - Saves the result of inverse FFT processing to the specified file name. Not valid when -s is specified. Not valid if -b is not specified

# About this benchmark shell script ( run_bench.sh )

It measures both the compile time of this project itself and the program execution.

In the first half, measure the processing speed of cargo(rust)

The second half measures the execution time of this program. The difference between f32 and f64 in FFT processing and the difference between DEBUG BUILD and RELEASE BUILD are interesting.

 1.  Run "cargo clean".
    - First, clean it.
 1.  Run "cargo check".
    - Measure the processing time of "cargo check".
    - If you have any downloads, try again.
 1.  After touching main.rs, Run "cargo check" again.
    - Measure only the processing time of main.rs.
 1.  Run "cargo build".
    - Measure the processing time of "cargo build".
 1.  After touching main.rs, Run "cargo build" again.
    - Measure only the processing time of main.rs.
 1.  Run "cargo build --release".
    - Measure the processing time of "cargo build --release".
 1.  After touching main.rs, Run "cargo build --release" again.
    - Measure only the processing time of main.rs.
 1. Run "fft_benchi" with various parameters.
    - ```-s``` ... skip_fft no_output
    - ```-s -w``` ... skip_fft wave_output_only
    - ``` ```   ... enable_fft no_output
    - ```-b```  ... enable_fft enable_fft_backword no_output
    - ```-w```  ... enable_fft wave_output
    - ```-b -x``` ... enable_fft enable_fft_backword wave_output wave_fft_backword_output
    - ```-6```    ... f64 enable_fft no_output
    - ```-6 -b``` ... f64 enable_fft enable_fft_backword no_output
    - ```-6 -w``` ... f64 enable_fft wave_output
    - ```-6 -w -b -x``` ... f64 enable_fft enable_fft_backword wave_output wave_fft_backword_output
 1. Run ```cat /proc/cpuinfo```
 1. Run ```free -h``` and ```cat /proc/meminfo```

Running this benchmark produces tmp.wav and tmp_fftbw.wav in the current directory.

tmp.wav is a file in which chords of A, C#, and E last for 180 seconds.

tmp_fftbw.wav is the data obtained by performing the inverse FFT processing after the FFT processing of the tmp.wav data.

Please try to hear.

# Summary of measurement results

The result of running on my server and Raspberry Pi 3 is in the result directory

 - <a href="result/AMD_Ryzen_7_1800X.txt">AMD_Ryzen_7_1800X.txt</a>
    - CPU : amd x86_64 8core/16thread
    - MEM : 64G
    - DISC: SSD
 - <a href="result/Raspberry_Pi_3_Model_B.txt">Raspberry_Pi_3_Model_B.txt</a>
    - CPU : arm7v 4core
    - MEM : 1G
    - DISC: SD card

## Compile speed comparison

|                                      | X86  | X86(CPU) | RPi 3B    | RPi 3B(CPU) |  RPi 3B : X86 |
|:---                                  |  ---:|      ---:|       ---:|         ---:|           ---:|
| cargo check 1                        | 3.66 | 355%     | 48.48     | 303%        |               |
| cargo check 2                        | 0.07 | 101%     | 0.61      | 102%        |               |
| cargo build 1                        | 6.15 | 488%     | 97.84     | 343%        |               |
| cargo build 2                        | 0.92 | 102%     | 12.91     | 100%        |               |
| cargo build --release 1              | 9.65 | 824%     | 177.4     | 285%        |               |
| cargo build --release 2              | 1.38 | 413%     | 18.76     | 278%        |               |
| cargo build 1 (cross arm7)           | 6.61 | 458%     | 96.10     | 345%        |               |
| cargo build 2 (cross arm7)           | 1.36 | 101%     | 12.91     | 100%        |               |
| cargo build --release 1 (cross arm7) | 9.35 | 845%     | 176.36    | 383%        |               |
| cargo build --release 2 (cross arm7) | 1.47 | 389%     | 18.08     | 278%        |               |

## Program speed comparison

|                                      | X86 (D)| RPi 3B (D)| X86 (R)| RPi 3B (R)| X86(D)/(R) | RPi 3B (D)/(R) | RPi 3B(D) : X86(D) | RPi 3B(R) : X86(R) |
|:---                                  |    ---:|       ---:|    ---:|       ---:|        ---:|            ---:|                ---:|                ---:|
| ```-s```                             | 4.44   | 31.38     | 0.29   | 3.45      |            |                |                    |                    |
| ```-s -w```                          | 4.02   | 21.77     | 0.31   | 3.58      |            |                |                    |                    |
| ``` ```                              | 8.33   | 35.60     | 0.37   | 4.38      |            |                |                    |                    |
| ```-b```                             | 13.98  | 57.38     | 0.45   | 5.28      |            |                |                    |                    |
| ```-w```                             | 8.80   | 38.24     | 0.38   | 4.56      |            |                |                    |                    |
| ```-b -x```                          | 15.18  | 65.47     | 0.49   | 6.25      |            |                |                    |                    |
| ```-6```                             | 6.52   | 37.12     | 0.59   | 5.85      |            |                |                    |                    |
| ```-6 -b```                          | 10.24  | 59.81     | 0.70   | 7.12      |            |                |                    |                    |
| ```-6 -w```                          | 7.00   | 41.56     | 0.61   | 6.05      |            |                |                    |                    |
| ```-6 -w -b -x```                    | 11.26  | 68.17     | 0.74   | 8.52      |            |                |                    |                    |

## total time

 - X86 Server
     - 135.55  ( 251% CPU )
 - RPi 3B
     - 19:33.49 ( 239% CPU )

## General comment

