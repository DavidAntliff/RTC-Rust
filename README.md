# The Ray Tracer Challenge - Rust

![workflow](https://github.com/DavidAntliff/RTC-Rust/actions/workflows/rust.yml/badge.svg)

## Conventions

This codebase uses the Left Hand Coordinate system.

## Building

```
 $ cargo build
```

## Development Notes

TODO


## Comparison with RTC-CPP

RTC-CPP perf stats:

```
$ perf stat cpp/ray_tracer_challenge/cmake-build-release/src/chapter10_perturbed_patterns > image.ppm

 Performance counter stats for 'cpp/ray_tracer_challenge/cmake-build-release/src/chapter10_perturbed_patterns':

          7,039.55 msec task-clock                #    1.000 CPUs utilized          
                20      context-switches          #    2.841 /sec                   
                 5      cpu-migrations            #    0.710 /sec                   
            52,504      page-faults               #    7.458 K/sec                  
    29,017,092,620      cycles                    #    4.122 GHz                    
    97,401,205,035      instructions              #    3.36  insn per cycle         
    13,300,398,903      branches                  #    1.889 G/sec                  
        78,188,392      branch-misses             #    0.59% of all branches        
   144,275,900,080      slots                     #   20.495 G/sec                  
    90,526,054,952      topdown-retiring          #     62.6% retiring              
    11,881,544,712      topdown-bad-spec          #      8.2% bad speculation       
    39,039,361,198      topdown-fe-bound          #     27.0% frontend bound        
     3,206,197,698      topdown-be-bound          #      2.2% backend bound         

       7.040438559 seconds time elapsed

       6.992133000 seconds user
       0.048000000 seconds sys


$ perf stat -e L1-dcache-load-misses,L1-icache-load-misses cpp/ray_tracer_challenge/cmake-build-release/src/chapter10_perturbed_patterns > image.ppm

 Performance counter stats for 'cpp/ray_tracer_challenge/cmake-build-release/src/chapter10_perturbed_patterns':

        16,584,237      L1-dcache-load-misses                                       
         9,472,442      L1-icache-load-misses                                       

       6.863133956 seconds time elapsed

       6.794779000 seconds user
       0.068027000 seconds sys
```

RTC-Rust, before caching inverted transform matrices:

```
$ perf stat target/release/chapter10_perturbed_patterns > image.ppm

 Performance counter stats for 'target/release/chapter10_perturbed_patterns':

          3,099.29 msec task-clock                #    1.000 CPUs utilized          
                10      context-switches          #    3.227 /sec                   
                 3      cpu-migrations            #    0.968 /sec                   
            37,371      page-faults               #   12.058 K/sec                  
    13,185,017,404      cycles                    #    4.254 GHz                    
    42,668,214,735      instructions              #    3.24  insn per cycle         
     5,032,009,860      branches                  #    1.624 G/sec                  
        52,054,965      branch-misses             #    1.03% of all branches        
    65,775,579,875      slots                     #   21.223 G/sec                  
    40,755,065,177      topdown-retiring          #     61.7% retiring              
     6,190,642,811      topdown-bad-spec          #      9.4% bad speculation       
    11,233,919,860      topdown-fe-bound          #     17.0% frontend bound        
     7,912,705,896      topdown-be-bound          #     12.0% backend bound         

       3.099916796 seconds time elapsed

       3.043758000 seconds user
       0.055995000 seconds sys


$ perf stat -e L1-dcache-load-misses,L1-icache-load-misses target/release/chapter10_perturbed_patterns > image.ppm

 Performance counter stats for 'target/release/chapter10_perturbed_patterns':

        11,160,830      L1-dcache-load-misses                                       
         5,510,678      L1-icache-load-misses                                       

       3.101941357 seconds time elapsed

       3.053837000 seconds user
       0.048028000 seconds sys
```


RTC-Rust, after caching inverted transform matrices:

```
$ perf stat target/release/chapter10_perturbed_patterns > image.ppm

 Performance counter stats for 'target/release/chapter10_perturbed_patterns':

          2,281.92 msec task-clock                #    1.000 CPUs utilized          
                 6      context-switches          #    2.629 /sec                   
                 2      cpu-migrations            #    0.876 /sec                   
            37,409      page-faults               #   16.394 K/sec                  
     9,850,975,494      cycles                    #    4.317 GHz                    
    31,482,049,466      instructions              #    3.20  insn per cycle         
     4,953,545,286      branches                  #    2.171 G/sec                  
        52,700,609      branch-misses             #    1.06% of all branches        
    49,167,467,990      slots                     #   21.547 G/sec                  
    29,500,480,794      topdown-retiring          #     60.0% retiring              
     5,205,967,198      topdown-bad-spec          #     10.6% bad speculation       
     8,483,798,398      topdown-fe-bound          #     17.3% frontend bound        
     5,977,221,598      topdown-be-bound          #     12.2% backend bound         

       2.282860171 seconds time elapsed

       2.218686000 seconds user
       0.064077000 seconds sys


$ perf stat -e L1-dcache-load-misses,L1-icache-load-misses target/release/chapter10_perturbed_patterns > image.ppm

 Performance counter stats for 'target/release/chapter10_perturbed_patterns':

        10,653,812      L1-dcache-load-misses                                       
         4,428,316      L1-icache-load-misses                                       

       2.324244012 seconds time elapsed

       2.264039000 seconds user
       0.060001000 seconds sys
```
