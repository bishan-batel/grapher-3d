/*
 * Kishan Patel
 * Oct 25
 * This class is used to connect and interact with a SQL database
 */
package bishan.grapher3d.database;

import java.io.Closeable;
import java.io.PrintStream;
import java.sql.*;
import java.util.ArrayList;
import java.util.Arrays;

import static java.lang.String.format;

/**
 * Wrapper class used to create, connect, and manage a single SQL database
 */
public class SQLDb implements Closeable
{

  private final String dbName;
  private Connection dbConn;

  /**
   * Used to connect to an existing database
   *
   * @param dbName Database
   */
  public SQLDb(String dbName) throws SQLException
  {
    this(dbName, "");
  }

  /**
   * Used to connect to SQL database with custom modifiers in URL (e.g.
   * ';create=true')
   *
   * @param dbName Database name
   * @param connectionURLAppend String to be appended to the URL connection
   * string
   */
  public SQLDb(String dbName, String connectionURLAppend) throws SQLException
  {
    this.dbName = dbName;
    setDbConn(connectionURLAppend);
  }

  /**
   * Shorthand to create new database and connect to it
   *
   * @param name DB Name
   * @return Created/Existing database
   */
  public static SQLDb createNewDb(String name) throws SQLException
  {
    return new SQLDb(name, ";create=true");
  }

  // Statement executors (Shorthands) -----------------------------------------
  /**
   * Shorthand for creating a new table
   *
   * @param table Table information
   */
  public void createTable(SQLTableInfo table) throws SQLException
  {
    exec(format("CREATE TABLE %s (%s)", table.getName(), table.getFullColumns()));
  }

  /**
   * Shorthand to select all records from a table
   *
   * @param table Table information
   */
  public String[][] selectAllFromTable(SQLTableInfo table) throws SQLException
  {
    return selectWhere(table, "1=1");
  }

  /**
   * Selects records in the specified table that validates the where clause
   *
   * @param table Table information
   * @param where SQL Statement for the "where" clause
   * @param preparedValues Values used for prepared statement (can be left empty
   * if not used)
   */
  public String[][] selectWhere(
    SQLTableInfo table,
    String where,
    Object... preparedValues
  ) throws SQLException
  {
    ResultSet result = execPreparedWithResult(
      // formats table name and where clause into SQL statement unsafely
      format(
        "SELECT * FROM %s WHERE %s",
        table.getName(),
        where
      ),
      // safely stores prepared values into statement
      preparedValues
    );

    // converts result set to arraylist then 2D array
    return to2dArray(resultToList2D(result, table.getTableHeaders()));
  }

  /**
   * Shorthand to drop a table from the database
   *
   * @param info Table Information
   */
  public void dropTable(SQLTableInfo info) throws SQLException
  {
    dropTable(info.getName());
  }

  /**
   * Shorthand to drop a table from the database
   *
   * @param name Table name
   */
  public void dropTable(String name) throws SQLException
  {
    exec(format("DROP TABLE %s", name));
  }

  /**
   * Shorthand to insert values into a table (all columns)
   *
   * @param table Table to insert into
   * @param vals Values to insert into table (order will match up to column)
   */
  public void insert(SQLTableInfo table, Object... vals) throws SQLException
  {
    var prepared = "?" + ",?".repeat(table.getColumnCount() - 1);
    execPrepared(format("INSERT INTO %s VALUES (%s)", table.getName(), prepared), vals);
  }

  /**
   * Shorthand for deleting information from a table that matches a where clause
   *
   * @param table Table to delete from
   * @param condition Where clause to determine if row should be deleted (omit
   * beginning WHERE)
   * @param vals Prepared values to be substituted in query
   */
  public void deleteWhere(SQLTableInfo table, String condition, Object... vals)
    throws SQLException
  {
    execPrepared(format("DELETE FROM %s WHERE %s", table.getName(), condition), vals);
  }

  // Statement executors (Raw) ------------------------------------------------
  /**
   * Used to execute a prepared statement with no result
   *
   * @param statement SQL Statement to be executed
   * @param vals Values to be substituted in statement
   */
  public void execPrepared(String statement, Object... vals) throws SQLException
  {
    // ignores result

    @SuppressWarnings("unused")
    var result = execPreparedWithResult(statement, vals);
  }

  /**
   * Used to execute prepared query & to retrieve result
   *
   * @param query SQL Statement to be executed
   * @param vals Values to be substituted in statement
   */
  public ResultSet execPreparedWithResult(String query, Object... vals) throws SQLException
  {
    PreparedStatement statement = dbConn.prepareStatement(query);

    for (int i = 0; i < vals.length; i++)
    {
      statement.setObject(i + 1, vals[i]);
    }

    // Prints to err stream if statement failed
    try
    {
      statement.execute();
      debugPrintQuery(query);
    }
    catch (SQLException e)
    {
      debugPrintQueryErr(query);
      throw e;
    }
    return statement.getResultSet();
  }

  /**
   * Executes SQL Query and returns result set
   *
   * @param query SQL Query to be executed
   */
  public ResultSet execWithResult(String query) throws SQLException
  {
    Statement statement = dbConn.createStatement();

    // Prints to err stream if statement failed
    try
    {
      statement.execute(query);
      debugPrintQuery(query);

      // (if statement ran) returns result set
      return statement.getResultSet();
    }
    catch (SQLException e)
    {
      debugPrintQueryErr(query);

      // throws inside catch block just because I want to be able to log when the error
      // happens above
      throw e;
    }
  }

  /**
   * Executes SQL Statement and does not record result
   *
   * @param statement SQL Statement to be executed
   */
  public void exec(String statement) throws SQLException
  {
    @SuppressWarnings("unused")
    ResultSet result = execWithResult(statement);
  }

  protected void debugPrintQuery(String q)
  {
    debugPrintQuery(false, q);
  }

  protected void debugPrintQueryErr(String q)
  {
    debugPrintQuery(true, q);
  }

  protected void debugPrintQuery(boolean err, String q)
  {
    PrintStream stream = err ? System.err : System.out;

    // debug prints the query
    stream.printf("[derby@%s] %s%n", dbName, q);
  }

  // Setters & Getters --------------------------------------------------------
  public String getDbName()
  {
    return dbName;
  }

  public Connection getDbConn()
  {
    return dbConn;
  }

  public void setDbConn() throws SQLException
  {
    setDbConn("");
  }

  public void setDbConn(String urlExtensions) throws SQLException
  {
    String connectionURL = "jdbc:derby:" + this.dbName + ";" + urlExtensions;
    dbConn = null;

    try
    {
      Class.forName("org.apache.derby.jdbc.EmbeddedDriver");
      dbConn = DriverManager.getConnection(connectionURL);
    }
    catch (ClassNotFoundException ex)
    {
      System.err.println("SQL Driver not found");
    }
    catch (SQLSyntaxErrorException sse)
    {
      sse.printStackTrace();
    }
  }

  public ArrayList<ArrayList<String>> resultToList2D(ResultSet rs, String... tableHeaders)
  {
    ArrayList<ArrayList<String>> data = new ArrayList<>();

    try
    {
      // loops through result query to store into data
      while (rs.next())
      {
        var row = new ArrayList<String>();

        // loops through each column to add to record
        for (String tableHeader : tableHeaders)
        {
          // gets data for column
          row.add(rs.getString(tableHeader));
        }

        data.add(row); // adds column to row
      }
    }
    catch (SQLException se)
    {
      System.err.println("SQL Err: Not able to get data");
    }

    return data;
  }

  public void ignoreSQLErr(IgnoreSQLRunnable runnable)
  {
    ignoreSQLErr(runnable, true);
  }

  public void ignoreSQLErr(IgnoreSQLRunnable runnable, boolean debug)
  {
    try
    {
      runnable.run();
    }
    catch (SQLException e)
    {
      if (debug)
      {
        e.printStackTrace();
      }
    }
  }

  @FunctionalInterface
  interface IgnoreSQLRunnable
  {

    void run() throws SQLException;
  }

  /**
   * Is object connected to the database;
   */
  public boolean isConnected()
  {
    if (dbConn == null)
    {
      return false;
    }
    try
    {
      return dbConn.isClosed();
    }
    catch (SQLException e)
    {
      return false;
    }
  }

  /**
   * Used to cleanly dispose of database connection, used when done with SQLDb
   * object
   */
  @Override
  public void close()
  {
    if (isConnected())
    {
      try
      {
        dbConn.close();
      }
      catch (SQLException e)
      {
        e.printStackTrace();
      }
    }
  }

  // Static helper methods ----------------------------------------------------
  /**
   * Turns array into string with values seperated by comma (none trailing)
   *
   * @param arr Array to stringify
   */
  public static String toStringSeperatedByCommas(Object[] arr)
  {
    // Empty array -> empty string
    if (arr.length == 0)
    {
      return "";
    }

    // Creates %s for every object then formats them in
    return String.format(
      ",%s"
        .repeat(arr.length)
        .substring(1), // removes leading comma
      arr
    );
  }

  /**
   * Converts string array list into static 2D Object Array
   *
   * @param data 2D Array list to convert, assumed to be not null
   */
  public static String[][] to2dArray(ArrayList<ArrayList<String>> data)
  {
    if (data.isEmpty())
    {
      return new String[0][0];
    }

    return data
      .stream()
      // converts each row to array
      .map(row -> row.toArray(String[]::new))
      // converts all rows into a static array
      .toArray(String[][]::new);
  }

  public static void main(String[] args) throws SQLException
  {
    var db = new SQLDb(SetupDB.DB_NAME);
    System.out.println(Arrays.deepToString(db.selectAllFromTable(SetupDB.USERS_TABLE)));
  }
}
