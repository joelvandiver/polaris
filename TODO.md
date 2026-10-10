# Polaris TODO

## Mathematics

- [ ] Build number theory module
  - [ ] Split number module into one file per type
    - Refer to claude bookmark.
  - [ ] Return error for negative Integer to WholeNumber
  - [ ] Map Integer zero to WholeNumber zero
  - [ ] Add Integer to NaturalNumber conversion
  - [ ] Add From<WholeNumber> for Integer
  - [ ] Add RationalNumber with nonzero denominator
  - [ ] Reduce RationalNumber to lowest terms
  - [ ] Add arithmetic ops for RationalNumber
  - [ ] Add From<Integer> for RationalNumber
  - [ ] Add RealNumber wrapper over f64
  - [ ] Decide how to model IrrationalNumber
  - [ ] Add ImaginaryNumber
  - [ ] Add ComplexNumber with arithmetic ops
- [ ] Build geometry module
  - [ ] Add slope of a Line
  - [ ] Add length of a Segment
  - [ ] Add midpoint of a Segment
  - [ ] Add intersection of two Lines
- [ ] Build an algebra problem generator
  - [ ] Evaluate LinearEquation at x
  - [ ] Solve LinearEquation for its root
  - [ ] Generate random linear equations
  - [ ] Format equations as readable text
- [ ] Build a calculator
  - [ ] Tokenize arithmetic expressions
  - [ ] Parse tokens into an expression tree
  - [ ] Evaluate expression trees

## Task

- [ ] Update tui
  - [ ] Render input box and result with ratatui
  - [ ] Add event loop for input, Enter, and quit
- [ ] Build a desktop ui
  - [ ] Pick a desktop UI framework
  - [ ] Scaffold desktop crate
- [ ] Verify the web ui
  - [ ] Run web tests against built wasm package
  - [ ] Add web test step to CI
