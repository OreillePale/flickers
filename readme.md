# Readme

## Motivation

### What is this crate
This rust crate contains functions to calculate time and frequency stability statistics such that Allan-like deviations.

For the moment only the Allan and Overlapping Allan deviations are available but I intend to add all the deviations that can be found in [stable32](http://www.stable32.com/), [allantools](https://allantools.readthedocs.io/en/latest/) or [sigmatheta](https://gitlab.com/fm-ltfb/SigmaTheta).

The goal is to unit test everything with `stable32`.

### Why another library for time statistics
I have two objectives:

 1. Learn rust to the point I am confident I can publicly realease some code for others to use.
 2. The aformentionned other libraries all lack something I intend to fix with my crate (speed, completeness, python wrapper).

## Examples
### Deviation calculation
Here is an example on how to calculate the Overlapping Allan deviation. For the moment the package only accepts phase as input.

```rust
fn main(){
    // generate phase data
    let phases = test_suite::generate_phase();

    let result = DevComputer::default()
        .with_phases(&phases)
        .with_tau0(1.)
        .with_afs(Afs::Decade)
        .with_noise_id(NoiseId::Default)
        .compute();

    println!("{:?}",result);
}
```

## What's next
### Features

 - [ ] code and publish a python wrapper
 - [ ] add all other deviations (MDev,TDev,Hdev,OHdev,MTIE,Theo1,TheoBr)
 - [ ] implement KLTS and KLTG method for calculating Three-Corner-Hat error bars. This can be only found in `sigmathea` for the moment and there is not python equivalent.
 - [ ] add noise generation functions
 - [ ] add Deviation fitting

### Code
 - [ ] add error handling
 - [ ] add parallel mode for slow functions
 - [ ] add optionnal `Serde` for `DevResult`

## Documentation and comments
My primarly source is the *Handbook of Frequency Stability Analysis* By W.J. Riley and the application notes found on Stable32's website. For the moment all the unit tests are compared to the values found in the former, especially Table 32.

The only deviation (yet) is the calculation of the error bars. [It appears](https://www.anderswallin.net/2020/12/fun-with-chi-squared/) that Stable32's code contains a typo when calculating the inverse $\chi_2$ cumulative distribution which is fixed in the current code.