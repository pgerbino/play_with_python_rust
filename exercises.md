Here are ten exercises to help you learn PyO3 in Rust:

### Exercise 1: Basic Python Function in Rust
**Objective:** Write a simple Rust function that adds two numbers and exposes it to Python.

**Steps:**
1. Create a new Rust project.
2. Add the `pyo3` crate to your `Cargo.toml` dependencies.
3. Write a Rust function named `add_numbers` that takes two `i32` arguments and returns their sum.
4. Use the `#[pyfunction]` and `#[pymodule]` macros to expose this function to Python.
5. Compile the Rust project into a Python module.
6. Test the function in Python by importing the module and calling the function.

### Exercise 2: Returning a Struct from Rust to Python
**Objective:** Create a Rust struct and return it as a Python object.

**Steps:**
1. Define a Rust struct `Point` with fields `x` and `y` of type `f64`.
2. Implement a method to create a new `Point` instance.
3. Use `#[pyclass]` and `#[pymethods]` to expose the struct and its methods to Python.
4. Write a Python script that creates a `Point` object and prints its fields.

### Exercise 3: Handling Python Exceptions in Rust
**Objective:** Write a Rust function that handles exceptions raised by Python functions.

**Steps:**
1. Write a Rust function that accepts a Python function as an argument.
2. Call the Python function from Rust, handling any exceptions that may occur using `PyErr`.
3. Test the Rust function from Python by passing a function that raises an exception.

### Exercise 4: Implementing a Rust Iterator in Python
**Objective:** Create a Rust struct that implements the `Iterator` trait and expose it to Python.

**Steps:**
1. Define a Rust struct `Counter` that implements the `Iterator` trait.
2. Expose the `Counter` struct to Python using PyO3.
3. Write a Python script that iterates over the `Counter` object.

### Exercise 5: Passing Structs as Function Arguments
**Objective:** Write a Rust function that accepts a struct as an argument and exposes it to Python.

**Steps:**
1. Define a Rust struct `Rectangle` with fields `width` and `height`.
2. Implement a function to calculate the area of the rectangle.
3. Expose the function and the struct to Python.
4. Test the function in Python by passing a `Rectangle` object.

### Exercise 6: Exposing Enums to Python
**Objective:** Create a Rust enum and expose it to Python.

**Steps:**
1. Define a Rust enum `Direction` with variants `Up`, `Down`, `Left`, and `Right`.
2. Implement a method to return the enum as a string.
3. Expose the enum and its method to Python.
4. Test the enum in Python by creating instances and printing their string representation.

### Exercise 7: Using Python's Standard Library in Rust
**Objective:** Write a Rust function that interacts with Python's `os` module.

**Steps:**
1. Write a Rust function that imports and uses Python's `os` module.
2. Use the `os` module to list files in a directory.
3. Return the list of files to Python.
4. Test the function in Python by passing a directory path.

### Exercise 8: Creating Python Classes in Rust
**Objective:** Create a Python class using Rust.

**Steps:**
1. Define a Rust struct and implement methods for it.
2. Use `#[pyclass]` and `#[pymethods]` to expose the struct and methods as a Python class.
3. Test the class in Python by creating instances and calling its methods.

### Exercise 9: Sharing Data Between Python and Rust
**Objective:** Write a Rust function that modifies a list shared with Python.

**Steps:**
1. Create a Rust function that takes a mutable reference to a list of integers.
2. Modify the list in Rust.
3. Test the function in Python by passing a list and checking the modifications.

### Exercise 10: Creating a Python Module with Multiple Functions
**Objective:** Write a Rust module with multiple functions and expose it to Python.

**Steps:**
1. Write several Rust functions, such as `add`, `subtract`, and `multiply`.
2. Use PyO3 to expose these functions as a Python module.
3. Test the module in Python by importing and using the functions.

These exercises will help you get hands-on experience with the basics of PyO3, including function calls, error handling, and data sharing between Rust and Python.
