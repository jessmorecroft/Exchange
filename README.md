| Language |     Part 1    |     Part 2    | 
|:--------:|:-------------:|:-------------:|
|  C++     | [![Build Status](https://dev.azure.com/garyedwardhughes/Exchange/_apis/build/status/3?branchName=master)](https://dev.azure.com/garyedwardhughes/Exchange/_build/latest?definitionId=3&branchName=master)              | [![Build Status](https://dev.azure.com/garyedwardhughes/Exchange/_apis/build/status/12?branchName=master)](https://dev.azure.com/garyedwardhughes/Exchange/_build/latest?definitionId=12&branchName=master)              | 
|  C#      | [![Build Status](https://dev.azure.com/garyedwardhughes/Exchange/_apis/build/status/Part%201%20-%20C%23?branchName=master)](https://dev.azure.com/garyedwardhughes/Exchange/_build/latest?definitionId=4&branchName=master)              | [![Build Status](https://dev.azure.com/garyedwardhughes/Exchange/_apis/build/status/Part%202%20-%20C%23?branchName=master)](https://dev.azure.com/garyedwardhughes/Exchange/_build/latest?definitionId=6&branchName=master) | 
|  F#      | [![Build Status](https://dev.azure.com/garyedwardhughes/Exchange/_apis/build/status/Part%201%20-%20F%23?branchName=master)](https://dev.azure.com/garyedwardhughes/Exchange/_build/latest?definitionId=5&branchName=master)              | [![Build Status](https://dev.azure.com/garyedwardhughes/Exchange/_apis/build/status/Part%202%20-%20F%23?branchName=master)](https://dev.azure.com/garyedwardhughes/Exchange/_build/latest?definitionId=7&branchName=master) |
|  Go      | [![Build Status](https://dev.azure.com/garyedwardhughes/Exchange/_apis/build/status/Part%201%20-%20Go?branchName=master)](https://dev.azure.com/garyedwardhughes/Exchange/_build/latest?definitionId=8&branchName=master)              | [![Build Status](https://dev.azure.com/garyedwardhughes/Exchange/_apis/build/status/Part%202%20-%20Go?branchName=master)](https://dev.azure.com/garyedwardhughes/Exchange/_build/latest?definitionId=9&branchName=master) |
|  Rust    | [![Build Status](https://dev.azure.com/garyedwardhughes/Exchange/_apis/build/status/Part%201%20-%20Rust?branchName=master)](https://dev.azure.com/garyedwardhughes/Exchange/_build/latest?definitionId=10&branchName=master)              | [![Build Status](https://dev.azure.com/garyedwardhughes/Exchange/_apis/build/status/Part%202%20-%20Rust?branchName=master)](https://dev.azure.com/garyedwardhughes/Exchange/_build/latest?definitionId=11&branchName=master)              |

The following benchmarks are run in Microsoft hosted Azure Pipeline agents so the performance can and will vary between runs depending on what machine they end up running on. Each group, Part 1, Part 2, etc is run on a single agent so the performance of implementations within that group can be compared with some meaning. Performance in the Azure environment is significantly worse than will be seen on modern desktop hardware.

## Part 1 Performance
[![Build Status](https://dev.azure.com/garyedwardhughes/Exchange/_apis/build/status/Part%201%20-%20Benchmark?branchName=master)](https://dev.azure.com/garyedwardhughes/Exchange/_build/latest?definitionId=13&branchName=master)

||100K orders|trades|
-|:-:|:-:|
|C#|0:00:00.553822|3368|
|Rust|0:00:01.650813|3368|
|F#|0:00:07.609196|3368|
|C++|0:00:27.460359|3368|
|Go|0:00:51.134921|3368|


## Part 2 Performance
[![Build Status](https://dev.azure.com/garyedwardhughes/Exchange/_apis/build/status/Part%202%20-%20Benchmark?branchName=master)](https://dev.azure.com/garyedwardhughes/Exchange/_build/latest?definitionId=14&branchName=master)

||100K orders|trades|
-|:-:|:-:|
|Rust|0:00:00.044862|3368|
|C++|0:00:00.076297|3368|
|Go|0:00:00.087019|3368|
|C#|0:00:00.183393|3368|
|F#|0:00:00.336191|3368|


||10M orders|trades|
-|:-:|:-:|
|Rust|0:00:05.410882|360131|
|C++|0:00:08.749452|360131|
|Go|0:00:11.526636|360131|
|C#|0:00:18.822558|360131|
|F#|0:00:27.309572|360131|


