import my_pyo3_project
import polars as pl

if __name__ == '__main__':
    print(my_pyo3_project.add_two_numbers(1, 2))
    print(pl.__version__)
    df = pl.DataFrame({"a": [1, 2, 3]})
    df2 = my_pyo3_project.from_polars(df)
    print(df2)