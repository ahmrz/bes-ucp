# Solving Fuel-Based Unit Commitment Problem Using Improved Binary Bald Eagle Search

This repository contains code used in the paper, ["Solving Fuel-Based Unit Commitment Problem Using Improved Binary Bald Eagle Search"](https://doi.org/10.1007/s42235-024-00591-7). The [Bald Eagle Search Algorithm (BES)](https://doi.org/10.1007/s10462-019-09732-5), developed by Hassan A. Alsattar et al. [1], belongs to the class of metaheuristic algorithms used to solve optimization problems. In this paper, a modified binary variation of the BES algorithm was introduced to solve the Fuel-Based Unit Commitment Problem, and the performance was compared with other metaheuristic algorithms.

## Abstract
The Unit Commitment Problem (UCP) corresponds to the planning of power generation schedules. The objective of the fuel-based unit commitment problem is to determine the optimal schedule of power generators needed to meet the power demand, which also minimizes the total operating cost while adhering to different constraints such as power generation limits, unit startup, and shutdown times. In this paper, four different binary variants of the Bald Eagle Search (BES) algorithm, were introduced, which used two variants using S-shape, U-shape, and V-shape transfer functions. In addition, the best-performing variant (using an S-shape transfer function) was selected and improved further by incorporating two binary operators: swap-window and window-mutation. This variation is labeled Improved Binary Bald Eagle Search (IBBESS2). All five variants of the proposed algorithm were successfully adopted to solve the fuel-based unit commitment problem using seven test cases of 4-, 10-, 20-, 40-, 60-, 80-, and 100-unit. For comparative evaluation, 34 comparative methods from existing literature were compared, in which IBBESS2 achieved competitive scores against other optimization techniques. In other words, the proposed IBBESS2 performs better than all other competitors by achieving the best average scores in 20-, 40-, 60-, 80-, and 100-unit problems. Furthermore, IBBESS2 demonstrated quicker convergence to an optimal solution than other algorithms, especially in large-scale unit commitment problems. The Friedman statistical test further validates the results, where the proposed IBBESS2 is ranked the best. In conclusion, the proposed IBBESS2 can be considered a powerful method for solving large-scale UCP and other related problems.




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
