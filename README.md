# fft_bench

In developing a product that runs on RPi3, I wanted performance indicators on RPi3.

I also found that there is a big difference in the speed of processing between debug and release builds of Rust.

So I made a simple program and got a benchmark.

You can run the benchmark with the following command steps.

```
$ git clone https://github.com/zuntan/fft_bench.git
$ cd fft_bench
$ cargo check
$ ( TIME="\nTIME R:%e S:%S U:%U P:%P CMD:%C"; \time bash run_bench.sh 2>&1 ) | tee result/result.txt
```

( I can't speak English, so I am writing the text with Google Translate. Please forgive me for strange sentences. )

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

 - Sample

```
$ cargo run  --release -- -l 180 -6 -w -b -x  ***
    Finished release [optimized] target(s) in 0.01s
     Running `target/release/fft_bench -l 180 -6 -w -b -x`
 INFO  fft_bench > f64           [true]
 INFO  fft_bench > skip_fft      [false]
 INFO  fft_bench > wav output    [tmp.wav]
 INFO  fft_bench > enable_fft_bw [true]
 INFO  fft_bench > wav output bw [tmp_fftbw.wav]
 INFO  fft_bench > wav len       [180] sec
 INFO  fft_bench > freqs         [440.0, 554.364990234375, 659.2550048828125]
 INFO  fft_bench > wav amp     [3.0]
 INFO  fft_bench > 30 sec done
 INFO  fft_bench > 60 sec done
 INFO  fft_bench > 90 sec done
 INFO  fft_bench > 120 sec done
 INFO  fft_bench > 150 sec done
 INFO  fft_bench > 180 sec done
 INFO  fft_bench > fft count       [1939]
 INFO  fft_bench > wav sample len  [7938048]
 INFO  fft_bench > wav file   len  [15884332]
 INFO  fft_bench > wav time    (1) [ 180.0011] sec
 INFO  fft_bench > proc time   (2) [   0.7240] sec
 INFO  fft_bench > (1) / (2)       [ 248.6203]
```

 - Thanks
    - [Hound](https://crates.io/crates/hound) A wav encoding and decoding library in Rust.
    - [chfft](https://crates.io/crates/chfft) Fastest Fourier Transform library implemented with pure Rust.
    - Many other library authors

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
| cargo check 1                        | 3.66 | 355%     | 48.48     | 303%        | 13.25         |
| cargo check 2                        | 0.07 | 101%     | 0.61      | 102%        | 8.71          |
| cargo build 1                        | 6.15 | 488%     | 97.84     | 343%        | 15.91         |
| cargo build 2                        | 0.92 | 102%     | 12.91     | 100%        | 14.03         |
| cargo build --release 1              | 9.65 | 824%     | 177.4     | 285%        | 18.38         |
| cargo build --release 2              | 1.38 | 413%     | 18.76     | 278%        | 13.59         |
| cargo build 1 (cross arm7)           | 6.61 | 458%     | 96.10     | 345%        | 14.54         |
| cargo build 2 (cross arm7)           | 1.36 | 101%     | 12.91     | 100%        | 9.49          |
| cargo build --release 1 (cross arm7) | 9.35 | 845%     | 176.36    | 383%        | 18.86         |
| cargo build --release 2 (cross arm7) | 1.47 | 389%     | 18.08     | 278%        | 12.30         |

 - At compile time, the X86 side is 8 to 14 times faster than RPi 3B. 13-18 times faster with a clean build

 - The difference between the debug build and the release build is about 1.5 times, which is not a big difference.

 - In my experience, compiling a large project on RPi 3B can take 10 minutes or more, so repeating builds on RPi 3B is not efficient. It can be said that development in a cross-compile environment is efficient.

## Program speed comparison

|                      | FFT | FFT-BW | FILE-OUT | f64 | X86 (D)| RPi 3B (D)| X86 (R)| RPi 3B (R)| X86(D)/(R) | RPi 3B (D)/(R) | RPi 3B(D) : X86(D) | RPi 3B(R) : X86(R) |
|:---                  |:---:|:---:   |:---:     |:---:|    ---:|       ---:|    ---:|       ---:|        ---:|            ---:|                ---:|                ---:|
| ```-s```             |     |        |          |     | 4.44   | 31.38     | 0.29   | 3.45      | 15.31      | 9.10           | 7.07               | 9.10               |
| ```-s -w```          |     |        | *        |     | 4.02   | 21.77     | 0.31   | 3.58      | 12.97      | 6.08           | 5.42               | 6.08               |
| ``` ```              | *   |        |          |     | 8.33   | 35.60     | 0.37   | 4.38      | 22.51      | 8.13           | 4.27               | 8.13               |
| ```-b```             | *   | *      |          |     | 13.98  | 57.38     | 0.45   | 5.28      | 31.07      | 10.87          | 4.10               | 10.87              |
| ```-w```             | *   |        | *        |     | 8.80   | 38.24     | 0.38   | 4.56      | 23.16      | 8.39           | 4.35               | 8.39               |
| ```-b -x```          | *   | *      | *        |     | 15.18  | 65.47     | 0.49   | 6.25      | 30.98      | 10.48          | 4.31               | 10.48              |
| ```-6```             | *   |        |          | *   | 6.52   | 37.12     | 0.59   | 5.85      | 11.05      | 6.35           | 5.69               | 6.35               |
| ```-6 -b```          | *   | *      |          | *   | 10.24  | 59.81     | 0.70   | 7.12      | 14.63      | 8.40           | 5.84               | 8.40               |
| ```-6 -w```          | *   |        | *        | *   | 7.00   | 41.56     | 0.61   | 6.05      | 11.48      | 6.87           | 5.94               | 6.87               |
| ```-6 -w -b -x```    | *   | *      | *        | *   | 11.26  | 68.17     | 0.74   | 8.52      | 15.22      | 8.00           | 6.05               | 8.00               |

 - Very large difference in execution speed between debug build and release build. [ X86(D)/(R), RPi 3B (D)/(R) ]
    - RPi3B has 6 to 10 times, X86 has 15 to 31 times!

 - *** Anyway, the debug build is running too slow. ***

 - The difference in execution speed between RPi3B and X86 is 6 to 10 times. [ RPi 3B(R) : X86(R) ]
    - With this method, there seems to be no significant difference depending on whether or not the file is output.
    - On RPi3B, f32 seems to be faster than f64. Probably because the OS is 32bit.
    - On x86, f64 seems to be faster than f32. Probably because the OS is 64bit. However, there is not much difference between f32 and f64 in the release build.

## total time

 - X86 Server
     -  135.55 sec | 02:15.55 ( 251% CPU )
 - RPi 3B
     - 1173.49 sec | 19:33.49 ( 239% CPU )
 - RPi 3B / X86 Server
     - 8.657 ( 865.7 % )

## General comment

 - Depending on the degree, rather than using Rust on RPi3B, it seems more efficient to develop it to some extent on an X86 machine and then run a cross-built one on RPi3B.
    - Otherwise, you'll have to take a lot of breaks and coffee at compile time. I'm bloated.

 - At that time, it is good to develop with RPi3B in mind that the performance difference is about 10 times.

 - If you use RPi3B, *** be sure to use the release build. *** The debug build runs too slowly.
    - I was about to give up on development with desperate lack of processing speed.

