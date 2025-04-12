import polars as pl


# df = pl.DataFrame({
#     'a': [1, 2, 3],
#     'b': [4, 5, 6],
# })
# print(df)


df = pl.DataFrame({
    "name": ["Alice", "Bob", "Charlie"],
    "age": [30, 45, 25],
    "address": [
        {"street": "123 Main St", "city": "New York", "zip": "10001"},
        {"street": "456 Maple Ave", "city": "Los Angeles", "zip": "90001"},
        {"street": "789 Elm Rd", "city": "Chicago", "zip": "60601"},
    ]
})
# df = df.unnest('address')
df = df.unnest()


df = pl.DataFrame({
    "name": ["Alice", "Bob", "Charlie"],
    "age": [30, 45, 25],
    "response.body.choices.message.content.address": [
        {"street": "123 Main St", "city": "New York", "zip": "10001"},
        {"street": "456 Maple Ave", "city": "Los Angeles", "zip": "90001"},
        {"street": "789 Elm Rd", "city": "Chicago", "zip": "60601"},
    ]
})
df = df.with_columns(
    pl.col("response.body.choices.message.content.address")
        .name.prefix_fields("response.body.choices.message.content.address.")
).unnest("response.body.choices.message.content.address.")
print(df)