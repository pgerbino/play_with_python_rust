use polars::prelude::*;
// need to import the dsl module to use the lazy functions
// the lazy functions are used to create a lazy execution plan
// so things like 'col' and 'filter' are not executed immediately
// you can execute the plan by calling unwrap on the lazy operation
// or by calling collect
// the difference between the two is that unwrap will return a DataFrame
// while collect will return a Series
use polars::lazy::dsl::*;

// create a struct named polar_adder
// add a method called add which takes two polars dataframe and returns a dataframe
struct PolarAdder;

impl PolarAdder {

    pub fn filter(&self, df_a: DataFrame, df_b: DataFrame) -> DataFrame {

        let df_a_lz = df_a.lazy();
        let bb = (df_b["column_b"]).clone();
        let my_expr = col("column_a").is_in(lit(bb));
        let filtered_df = df_a_lz
            .filter(my_expr)
            .collect().unwrap();
        filtered_df
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter() {
        // Create test dataframes
        let df_a = DataFrame::new(vec![
            Series::new("column_a", &[1, 2, 3]),
            Series::new("column_b", &[4, 5, 6]),
        ])
        .unwrap();

        let df_b = DataFrame::new(vec![
            Series::new("column_b", &[4, 5, 6]),
            Series::new("column_c", &[7, 8, 9]),
        ])
        .unwrap();

        // Call the filter method
        let polar_adder = PolarAdder;
        let filtered_df = polar_adder.filter(df_a, df_b);

        // Assert the filtered dataframe
        let expected_df = DataFrame::new(vec![
            Series::new("column_a", &[1, 2, 3]),
            Series::new("column_b", &[4, 5, 6]),
        ])
        .unwrap();

        assert_eq!(filtered_df, expected_df);
    }
}
