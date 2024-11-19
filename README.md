# Solving Fuel-Based Unit Commitment Problem Using Improved Binary Bald Eagle Search

This repository contains code used in the paper, ["Solving Fuel-Based Unit Commitment Problem Using Improved Binary Bald Eagle Search"](https://doi.org/10.1007/s42235-024-00591-7). The [Bald Eagle Search Algorithm (BES)](https://doi.org/10.1007/s10462-019-09732-5), developed by Hassan A. Alsattar et al. [1], belongs to the class of metaheuristic algorithms used to solve optimization problems. In this paper, a modified binary variation of the BES algorithm was introduced to solve the Fuel-Based Unit Commitment Problem, and the performance was compared with other metaheuristic algorithms.

## Abstract
The Unit Commitment Problem (UCP) corresponds to the planning of power generation schedules. The objective of the fuel-based unit commitment problem is to determine the optimal schedule of power generators needed to meet the power demand, which also minimizes the total operating cost while adhering to different constraints such as power generation limits, unit startup, and shutdown times. In this paper, four different binary variants of the Bald Eagle Search (BES) algorithm, were introduced, which used two variants using S-shape, U-shape, and V-shape transfer functions. In addition, the best-performing variant (using an S-shape transfer function) was selected and improved further by incorporating two binary operators: swap-window and window-mutation. This variation is labeled Improved Binary Bald Eagle Search (IBBESS2). All five variants of the proposed algorithm were successfully adopted to solve the fuel-based unit commitment problem using seven test cases of 4-, 10-, 20-, 40-, 60-, 80-, and 100-unit. For comparative evaluation, 34 comparative methods from existing literature were compared, in which IBBESS2 achieved competitive scores against other optimization techniques. In other words, the proposed IBBESS2 performs better than all other competitors by achieving the best average scores in 20-, 40-, 60-, 80-, and 100-unit problems. Furthermore, IBBESS2 demonstrated quicker convergence to an optimal solution than other algorithms, especially in large-scale unit commitment problems. The Friedman statistical test further validates the results, where the proposed IBBESS2 is ranked the best. In conclusion, the proposed IBBESS2 can be considered a powerful method for solving large-scale UCP and other related problems.

[Complete journal paper](Paper.pdf).

## Datasets used

UCP datasets are selected based on the number of units, i.e. 4-, 10-, 20-, 40-, 60-, 80-, and 100-units [2]. The 4-unit properties and hourly demands are listed below:

### 4-unit properties
| Properties | Unit 1 | Unit 2 | Unit 3 | Unit 4 |
| :------------- | :------------- | :------------- | :------------- | :------------- |
| $P$<sub>max</sub> (MW) | 300 | 250 | 80 | 60 |
| $P$<sub>min</sub> (MW) | 75 | 60 | 25 | 20 |
| $a$ (\$/h) | 648.74 | 585.62 | 213.0 | 252.0 |
| $b$ (\$/MWh) | 16.83 | 16.95 | 20.74 | 23.6 |
| $c$ (\$/MW<sup>2</sup>h) | 0.0021 | 0.0042 | 0.0018 | 0.0034 |
| Min Uptime (h) | 4 | 3 | 2 | 1 |
| Min Downtime (h) | 5 | 5 | 4 | 1 |
| Hot Start Cost (\$) | 500 | 170 | 150 | 0 |
| Cold Start Cost (\$) | 1100 | 400 | 350 | 0.02 |
| Cold Start Hours (h) | 5 | 5 | 4 | 0 |
| Initial Status (h) | 8 | 8 | -5 | -6 |

### 4-unit hourly demand
| Hour | Demand |
| :------------- | :------------- |
| 1 | 450 |
| 2 | 530 |
| 3 | 600 |
| 4 | 540 |
| 5 | 400 |
| 6 | 280 |
| 7 | 290 |
| 8 | 500 |

The 10-unit properties and hourly demands are listed below:

### 10-unit properties
| Properties | Unit 1 | Unit 2 | Unit 3 | Unit 4 | Unit 5 | Unit 6 | Unit 7 | Unit 8 | Unit 9 | Unit 10 |
| :------------- | :------------- | :------------- | :------------- | :------------- | :------------- | :------------- | :------------- | :------------- | :------------- | :------------- |
| $P$<sub>max</sub> (MW) | 455 | 455 | 130 | 130 | 162 | 80 | 85 | 55 | 55 | 55 |
| $P$<sub>min</sub> (MW) | 150 | 150 | 20 | 20 | 25 | 20 | 25 | 10 | 10 | 10 |
| $a$ (\$/h) | 1000.0 | 970.0 | 700.0 | 680.0 | 450.0 | 370.0 | 480.0 | 660.0 | 665.0 | 670.0 |
| $b$ (\$/MWh) | 16.19 | 17.26 | 16.6 | 16.5 | 19.7 | 22.26 | 27.74 | 25.92 | 27.27 | 27.79 |
| $c$ (\$/MW<sup>2</sup>h) | 0.00048 | 0.00031 | 0.002 | 0.00211 | 0.00398 | 0.00712 | 0.00079 | 0.00413 | 0.00222 | 0.00173 |
| Min Uptime (h) | 8 | 8 | 5 | 5 | 6 | 3 | 3 | 1 | 1 | 1 |
| Min Downtime (h) | 8 | 8 | 5 | 5 | 6 | 3 | 3 | 1 | 1 | 1 |
| Hot Start Cost (\$) | 4500 | 5000 | 550 | 560 | 900 | 170 | 260 | 30 | 30 | 30 |
| Cold Start Cost (\$) | 9000 | 10000 | 1100 | 1120 | 1800 | 340 | 520 | 60 | 60 | 60 |
| Cold Start Hours (h) | 5 | 5 | 4 | 4 | 4 | 2 | 2 | 0 | 0 | 0 |
| Initial Status (h) | 8 | 8 | -5 | -5 | -6 | -3 | -3 | -1 | -1 | -1 |

### 10-unit hourly demand
| Hour | Demand |
| :------------- | :------------- |
| 1 | 700 |
| 2 | 750 |
| 3 | 850 |
| 4 | 950 |
| 5 | 1000 |
| 6 | 1100 |
| 7 | 1150 |
| 8 | 1200 |
| 9 | 1300 |
| 10 | 1400 |
| 11 | 1450 |
| 12 | 1500 |
| 13 | 1400 |
| 14 | 1300 |
| 15 | 1200 |
| 16 | 1050 |
| 17 | 1000 |
| 18 | 1100 |
| 19 | 1200 |
| 20 | 1400 |
| 21 | 1300 |
| 22 | 1100 |
| 23 | 900 |
| 24 | 800 |

For problems larger than the 10-unit problem, such as the 20-unit problem, the unit properties remain the same, except that the units were now repeated twice. Similarly, for the 40-unit problem, the 10-units were repeated four times, for the 60-unit problem, they were repeated six times, and so forth. For problems larger than the 10-unit problem, the operating schedule window remained unchanged at twenty-four hours as compared with the 10-unit problem except that the demand per hour is multiplied by the increase in the number of units (eg: twice for 20 units, four times for 40-units and so forth).

## Instructions to run code

Make sure you have Rust compiler installed on your computer. See [Rust website](https://www.rust-lang.org/tools/install) for more details. Clone the repository and navigate to the project root folder and type in the terminal:

```console
cargo run --release
```


## Citing the paper

BibTeX:

```console
@article{ali2024solving,
  title={Solving Fuel-Based Unit Commitment Problem Using Improved Binary Bald Eagle Search},
  author={Ali, Sharaz and Al-Betar, Mohammed Azmi and Nasor, Mohamed and Awadallah, Mohammed A},
  journal={Journal of Bionic Engineering},
  pages={1--25},
  year={2024},
  publisher={Springer}
}

```



| Type  | Citation |
| :------------- | :------------- |
| MLA  | Ali, Sharaz, et al. "Solving Fuel-Based Unit Commitment Problem Using Improved Binary Bald Eagle Search." _Journal of Bionic Engineering_ (2024): 1-25. |
| APA  | Ali, S., Al-Betar, M. A., Nasor, M., & Awadallah, M. A. (2024). Solving Fuel-Based Unit Commitment Problem Using Improved Binary Bald Eagle Search. _Journal of Bionic Engineering_, 1-25. |
| Chicago  | Ali, Sharaz, Mohammed Azmi Al-Betar, Mohamed Nasor, and Mohammed A. Awadallah. "Solving Fuel-Based Unit Commitment Problem Using Improved Binary Bald Eagle Search." _Journal of Bionic Engineering_ (2024): 1-25. |
| Harvard | Ali, S., Al-Betar, M.A., Nasor, M. and Awadallah, M.A., 2024. Solving Fuel-Based Unit Commitment Problem Using Improved Binary Bald Eagle Search. _Journal of Bionic Engineering_, pp.1-25. |
| Vancouver | Ali S, Al-Betar MA, Nasor M, Awadallah MA. Solving Fuel-Based Unit Commitment Problem Using Improved Binary Bald Eagle Search. Journal of Bionic Engineering. 2024 Nov 16:1-25. |


## References

1. H.A. Alsattar, A.A. Zaidan, and B.B. Zaidan, 2020. "Novel meta-heuristic bald eagle search optimisation algorithm." _Artificial Intelligence Review_, 53, pp.2237-2264.
2. L. K. Panwar, S. Reddy, A. Verma, B.K. Panigrahi, and R. Kumar, 2018. "Binary grey wolf optimizer for large scale unit commitment problem." _Swarm and Evolutionary Computation_, 38, pp.251–266.
