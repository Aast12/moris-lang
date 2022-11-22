let df: DataFrame = read_csv("train.csv");

print_names(df);
println(df);

scatter(select(df, "LotArea"), select(df, "SalePrice"));