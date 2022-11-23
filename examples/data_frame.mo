let df: DataFrame = read_csv("train.csv");

describe(df);

set_caption("Year Built vs Lot Area chart");
set_x_title("YearBuilt");
set_y_title("LotArea");
set_plot_out("scatter.png");

set_y_bounds(0, 30000);
set_x_bounds(1910, 2022);

scatter(select(df, "YearBuilt"), select(df, "LotArea"));

