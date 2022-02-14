package bishan.grapher3d;

import bishan.grapher3d.handlers.APIHandler;
import bishan.grapher3d.handlers.FileHandler;
import bishan.grapher3d.handlers.auth.AuthHandler;
import bishan.grapher3d.request.RequestInfo;
import com.sun.net.httpserver.HttpExchange;
import com.sun.net.httpserver.HttpServer;

import javax.swing.*;
import java.io.IOException;
import java.net.InetSocketAddress;
import java.util.concurrent.Executors;

/**
 * @author bishan
 */
public class MainServer
{

  /**
   * TODO Change this to whatever the build output of javascript is on the
   * server computer
   */
  // public static final String JS_BUILD_DIR =
  // "/home/bishan/code/web/grapher-3d/client/build";
  public static final String JS_BUILD_DIR = "src/js-build";

  /**
   * Path to javascript project's build output Needed for server to know where
   * to find website files.
   */
  public static int PORT = 3000;
  private HttpServer server;
  private final FileHandler fileHandler;

  private AuthHandler authHandler;
  private APIHandler apiHandler;

  public MainServer()
  {
    fileHandler = new FileHandler(this, JS_BUILD_DIR, new String[]
    {
      ".wasm",
      ".png",
      ".jpg",
      ".jpeg",
      ".svg",
      ".html",
      ".js",
      ".chunk.js",
      ".css",
      ".txt",
      ".vert",
      ".glsl",
      ".frag",
      ".json",
    });

    authHandler = new AuthHandler(this);
    apiHandler = new APIHandler(this);
  }

  /**
   * Starts HTTP server
   *
   * @throws IOException Throws if failed to start server
   */
  public void start() throws IOException
  {
    System.out.println("Starting server. . .");

    // Create HTTP Server
    server = HttpServer.create(new InetSocketAddress(PORT), 0);

    // Add context for '/' (all routes) to handle method
    server.createContext("/", ex ->
    {
      // attempts to handle HTTPExchange
      try
      {
        handle(ex);
      }
      catch (Exception e)
      {
        // logs error to stderr (if any)
        e.printStackTrace();
      }
    });

    // Sets thread pool for execution
    server.setExecutor(Executors.newFixedThreadPool(5));

    // Start Server
    server.start();

    System.out.printf(
      "Server started on http://localhost:%d %n",
      server.getAddress().getPort());
  }

  /**
   * Context to handle requests from clients
   *
   * @param ex Exchange information
   */
  private void handle(HttpExchange ex) throws IOException
  {
    var reqInfo = new RequestInfo(ex);
    String reqHead = reqInfo.getRouteSection(0);

    System.out.println(reqInfo.getRoute());
    if ("api".equals(reqHead))
    {
      apiHandler.handle(reqInfo);
      return;
    }
    else if ("auth".equals(reqHead))
    {
      authHandler.handle(reqInfo);
      return;
    }

    // If neither API nor authentication, treat request as file transfer
    fileHandler.handle(reqInfo);
  }

  // Getters and setters
  public HttpServer getServer()
  {
    return server;
  }

  public AuthHandler getAuthHandler()
  {
    return authHandler;
  }

  public static void main(String[] args) throws Exception
  {
    new MainServer().start();
  }
}
