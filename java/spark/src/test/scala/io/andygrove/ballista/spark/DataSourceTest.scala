package io.andygrove.ballista.spark

import org.junit.Test
import org.apache.spark.sql.SparkSession

class DataSourceTest {

  @Test
  def testSomething() {

    val spark = SparkSession.builder()
      .master("local[*]")
      .getOrCreate()

    val df = spark.read
      .format("io.andygrove.ballista.spark")
      .option("table", "alltypes_plain")
      .load()

    val query = df
      .select("a", "b")
      .filter("c < d")

    query.explain()

    val results = query.collect()

  }
}