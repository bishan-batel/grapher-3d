/*
 * Kishan Patel
 * Oct 29.
 * This class is used to install the database and neccesary parts onto the
 * users machine
 */
package bishan.grapher3d.database;

import java.sql.SQLException;
import java.util.Arrays;

public final class SetupDB
{

  public static final String DB_NAME = "3dGraphs";
  public static final SQLTableInfo USERS_TABLE = new SQLTableInfo(
    "Users", // name
    "id int NOT NULL PRIMARY KEY", // ID
    "email varchar(254) NOT NULL", // email
    "passwordHash varchar(256)", // pass hash
    "passwordSalt varchar(256)" // salt
  );

  public static final SQLTableInfo GRAPHS_TABLE = new SQLTableInfo(
    "Graph", // name
    "id int NOT NULL PRIMARY KEY", // ID
    "owner int NOT NULL", // owner ID
    "graphName varchar(44) NOT NULL", // graph name
    "description varchar(280)", // description
    "animate boolean" // animate (bool)
  );

  public static final SQLTableInfo GRAPH_EQUATIONS_TABLE = new SQLTableInfo(
    "GraphEquations", // name
    "zIndex int NOT NULL", // Z Index
    "ownerGraph int NOT NULL", // owner ID
    "equation varchar(255)", // equation ASCII
    "disabled boolean" // is disabled
  );

  public static void main(String[] args) throws SQLException
  {
    // Creates new database & automatically closes it
    try ( SQLDb db = SQLDb.createNewDb(DB_NAME))
    {

      // Stream of all tables
      Arrays.stream((new SQLTableInfo[]
      {
        USERS_TABLE, GRAPHS_TABLE, GRAPH_EQUATIONS_TABLE
      })
      // Iterates through all and attempts to create
      ).forEach(table ->
      {
        // drops table and creates
        db.ignoreSQLErr(() -> db.dropTable(table));
        db.ignoreSQLErr(() -> db.createTable(table));
      });

      System.out.println(USERS_TABLE.getColumn(0));
    }
  }
}
