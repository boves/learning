# working through: https://pandas.pydata.org/pandas-docs/stable/getting_started/intro_tutorials/02_read_write.html
import pandas as pd

url = "https://raw.githubusercontent.com/pandas-dev/pandas/main/doc/data/titanic.csv"
titanic = pd.read_csv(url)

titanic.head(8)

titanic["PassengerId"] # fixed.