import polars as pl

# df = pl.DataFrame({"a": [[1, 2], [3, None]]})
# o = (df
#      .with_columns(pl.col("a").list.contains(pl.lit(None)))
# #    .with_columns(pl.col("a").list.contains(None))
# )
# print(o)

# issue 4
df = pl.DataFrame(
    {
        "value": [1, 2, 1, 2],
        "group": [0, 0, 1, 1],
    }
)
gb = df.group_by(pl.col.group)
out = gb.agg(pl.col.value.quantile(pl.col.group.first()))
print(gb)
print(out)

out = gb.agg(pl.col.value.quantile(0))
print(out)

out = gb.agg(pl.col.value.quantile(1))
print(out)

# df = df.join(df)