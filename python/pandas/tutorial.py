# working through: https://pandas.pydata.org/pandas-docs/stable/getting_started/intro_tutorials/02_read_write.html
import pandas as pd

url = "https://raw.githubusercontent.com/pandas-dev/pandas/main/doc/data/titanic.csv"
titanic = pd.read_csv(url)

# Data exploration
titanic.head(8)
titanic["PassengerId"] # fixed.
titanic["Survived"]
titanic["Pclass"]
titanic.dtypes

ages = titanic["Age"]
ages.head()
type(titanic["Age"])
titanic["Age"].shape

age_sex = titanic[["Age", "Sex"]]
age_sex.head()

above_35 = titanic[titanic["Age"] > 35]
above_35.head()