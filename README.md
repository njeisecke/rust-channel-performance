               Debug         Release
               ---------------------
    Linux (x64, VM):
     std       882.259748ms  609.510615ms
     crossbeam 68.224804ms   53.862752ms
    
    Windows 10 (x64, native):
     std       10.548403ms   6.56759ms
     crossbeam 12.352206ms   5.800076ms
    
    macOS (x64):
     std       18.407234ms   111.287036ms
     crossbeam 13.771614ms   5.219074ms

    macOS (m1):
     std       87.059034ms   55.363459ms
     crossbeam 10.557704ms   4.978034ms

* Why is std in the *release* build so much slower on macOS *x64* (m1 looks
  reasonable)?

* Doesn't std::mpsc use the same implementation like crossbeam since some time?
  I'm surpised by the significant difference (no so much on Windows, but on m1 and Linux)

rustc 1.78.0
crossbeam 0.8.4

I know that benchmarking debug builds usually doesn't make that much sense,
however in this case of a release build being 5x slower than a debug build I
found this to be of interest.

Any ideas?
