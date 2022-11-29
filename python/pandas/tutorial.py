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
class_23 = titanic[titanic["Pclass"].isin([2, 3])]
class_23.head()
age_no_na = titanic[titanic["Age"].notna()]
age_no_na.head()
age_no_na.shape
adult_names = titanic.loc[titanic["Age"] > 35]