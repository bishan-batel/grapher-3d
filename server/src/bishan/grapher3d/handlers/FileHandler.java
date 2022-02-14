package bishan.grapher3d.handlers;

import bishan.grapher3d.MainServer;
import bishan.grapher3d.request.RequestInfo;
import com.sun.net.httpserver.HttpExchange;

import java.io.*;
import java.net.URI;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.*;

import static java.lang.String.format;

public class FileHandler {

  /**
   * Key value pairs of all mime types
   */
  public static final Map<String, String> MIME_TYPES;
  public static final byte[] FNF_ERROR = "404 File Not Found".getBytes();

  // Static constructor to initialize types in MIME_TYPES
  static {
    MIME_TYPES = new HashMap<>();

    // Adds all pairs of file extensions to their mime types
    // Text
    MIME_TYPES.put(".txt", "text/plain");
    MIME_TYPES.put(".html", "text/html; charset=utf8");
    MIME_TYPES.put(".css", "text/css");
    MIME_TYPES.put(".js", "text/javascript");
    MIME_TYPES.put(".json", "application/json");

    // Images
    MIME_TYPES.put(".png", "image/png");
    MIME_TYPES.put(".svg", "image/svg+xml");

    // Other
    MIME_TYPES.put(".zip", "application/zip");
    MIME_TYPES.put(".wasm", "application/wasm");
  }

  private final MainServer main;
  private final String publicPath;
  private final String[] allowedFiles;

  /**
   * Creates new File Handler
   *
   * @param main         reference to main server
   * @param publicPath
   * @param allowedFiles
   */
  public FileHandler(MainServer main, String publicPath, String[] allowedFiles) {
    this.main = main;
    this.publicPath = publicPath;
    this.allowedFiles = allowedFiles;
  }

  /**
   * Handles request for a file
   *
   * @param info Exchange info
   */
  public void handle(RequestInfo info) throws IOException {
    HttpExchange ex = info.getEx();
    String route = info.getRoute();
    String requestedFiletype = "txt";

    // verifies that the request file type is valid
    boolean requestFiletypeIsOK = false;

    for (String fileType : allowedFiles) {
      if (route.endsWith(fileType)) {
        requestFiletypeIsOK = true;
        requestedFiletype = fileType;
        break;
      }
    }

    // if filetype is not valid then assume request is for react-router
    if (!requestFiletypeIsOK) {
      route = "/index.html";
      requestedFiletype = ".html";
    }

    // get file bytes
    URI fileURI;

    // Attempts to grab file URI
    try {
      fileURI = fileURI(route);
    } catch (FileNotFoundException fnf) {
      // if file does not exist reroute to index page
      route = "/index.html";
      requestedFiletype = ".html";
      fileURI = fileURI(route);
    }

    byte[] buffer = Files.readAllBytes(Path.of(fileURI));
    var file = new File(fileURI);

    // Mime type header to tell client what file type
    ex.getResponseHeaders().add(
        "Content-Type",
        MIME_TYPES.get(requestedFiletype));

    // Tell browser to display inline
    ex.getResponseHeaders().add(
        "Content-Disposition",
        "inline; filename=" + file.getName());

    // Send respoonse headers
    ex.sendResponseHeaders(200, buffer.length);

    // Prepare to write bytes to response
    try (OutputStream os = info.getResponseBody()) {
      // writes file bytes buffer as response
      os.write(buffer);
    } catch (IOException e) {
      // print IO exception if failed, this should not happen unless the client
      // or server had an internet connection error
      e.printStackTrace();
    }
  }

  /**
   * Maps path to a file URI on server
   *
   * @param path relative path to file on server
   * @return URI from file
   */
  public URI fileURI(String path) throws FileNotFoundException {
    // gets full path & URI to file on server
    String fileRelativePath = publicPath + path;
    URI fileURI = Paths.get(fileRelativePath).toUri();

    // Creates file object to URI
    // If file does not exist or cannot read then throw error
    var file = new File(fileURI);
    if (!file.exists() || !file.canRead()) {
      throw new FileNotFoundException(format("File '%s' does not exist.", fileURI));
    }

    // return valid file URI
    return fileURI;
  }

  // Accessors & Mutators
  public MainServer getMain() {
    return main;
  }

  public String getPublicPath() {
    return publicPath;
  }

  public String[] getAllowedFiles() {
    return allowedFiles;
  }
}
