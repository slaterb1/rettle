# rettle
This library is an ETL (**E**xtract, **T**ransfrom, **L**oad), with inspiration drawn from Keras (https://keras.io/), to allow a "Brew Master" to define any order of operations for data transformations.

## Types
rettle (word play on kettle) defines the following Types for any project to "Brew" data:
- **Pot:** container that holds the set of instructions for data sources, sinks, and transforms (*See Recipe*)
- **Recipe:** order of operations for data inputs, transformation, combining data sources, removing fields, and sending data to final destinations
- **Tea:** data object that is being transformed in the ETL pipeline
- **Brewer:** worker / channel processing Tea

## Recipe Types
- **Fill:** data input source
- **Transfuse:** combine data from multiple sources defined before this step
- **Steep:** data transformation step
- **Skim:** remove a field (or Tea object)
- **Pour:** data output destination

## Traits
- **Ingredient:** identifies types that can be put in recipe
