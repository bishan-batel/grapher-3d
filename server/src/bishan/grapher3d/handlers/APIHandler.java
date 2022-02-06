package bishan.grapher3d.handlers;

import bishan.grapher3d.MainServer;
import bishan.grapher3d.database.SQLDb;
import bishan.grapher3d.database.SetupDB;
import bishan.grapher3d.handlers.auth.AuthHandler;
import bishan.grapher3d.request.RequestInfo;
import com.sun.net.httpserver.HttpExchange;

import javax.naming.NoPermissionException;
import java.io.IOException;
import java.io.OutputStream;
import java.sql.SQLException;
import java.util.*;

import static java.lang.String.format;

public class APIHandler {
	private MainServer main;
	private AuthHandler auth;

	public APIHandler(MainServer main) {
		this.main = main;
		auth = main.getAuthHandler();
	}

	public void handle(RequestInfo req) throws IOException {
		HttpExchange ex = req.getEx();
		String subRoute = req.getRouteSection(1);

		switch (subRoute) {
			case "req":
				handleGraphReq(req);
				return;
			case "graphs":
				getGraphNames(req);
				return;
			case "update":
				updateGraph(req);
				return;
			case "create":
				createGraph(req);
				return;
			case "delete":
				deleteGraph(req);
				return;
			default:
				byte[] err = format("Invalid route '%s'", subRoute).getBytes();
				OutputStream body = ex.getResponseBody();
				ex.sendResponseHeaders(404, err.length);
				body.write(err);
				body.close();
		}
	}

	public void getGraphNames(RequestInfo req) throws IOException {
		HttpExchange ex = req.getEx();
		OutputStream body = req.getResponseBody();
		String sessionId = req.getSessionId();

		if (!auth.getSessionManager().isValidToken(sessionId)) {
			ex.sendResponseHeaders(401, 0);
			body.close();
			return;
		}

		Integer uid = auth.getSessionManager().getUID(sessionId);
		String[][] graphs;

		try (var db = new SQLDb(SetupDB.DB_NAME)) {
			// retrieves graphs
			graphs = db.selectWhere(SetupDB.GRAPHS_TABLE, "owner=?", uid);
		} catch (SQLException e) {
			// Send internal server error
			byte[] msg = "Internal server error".getBytes();
			ex.sendResponseHeaders(500, msg.length);
			body.write(msg);
			body.close();
			e.printStackTrace();
			return;
		}

		var json = new StringBuilder();
		json.append("{\"graphs\":[");

		// loops through all graphs and appends name to json array
		for (String[] graph : graphs) {
			String name = graph[2];
			json.append("\"");
			json.append(name);
			json.append("\",");
		}
		json.deleteCharAt(json.lastIndexOf(",")); // remove trailing comma
		json.append("]}");

		// response with 200 OK and JSON bytes
		byte[] response = json.toString().getBytes();
		ex.sendResponseHeaders(200, response.length);
		body.write(response);
		body.close();
	}

	public void handleGraphReq(RequestInfo req) throws IOException {
		HttpExchange ex = req.getEx();
		OutputStream body = req.getResponseBody();
		String sessionId = req.getSessionId();

		// Request Arguments
		Map<String, String> reqBody = req.getRequestBody();
		String graphName = reqBody.get("name");

		try {
			// check if permitted or not
			if (!auth.isAuthorizedForGraph(sessionId, graphName))
				throw new NoPermissionException();
		} catch (SQLException e) {
			// Send internal server error
			ex.sendResponseHeaders(500, 0);
			body.close();
			e.printStackTrace();
			return;
		} catch (ArrayIndexOutOfBoundsException | NoPermissionException | IllegalArgumentException e) {
			// if not permitted send unauthorized error
			ex.sendResponseHeaders(401, 0);
			body.close();
			return;
		}

		// get owner ID (we know its valid because we checked authorization)
		var ownerId = (int) auth.getSessionManager().getUID(sessionId);
		try (var db = new SQLDb(SetupDB.DB_NAME)) {
			String[] graphRow = db.selectWhere(
				SetupDB.GRAPHS_TABLE,
				"owner=? AND graphName=?",
				ownerId,
				graphName
			)[0];

			int graphId = Integer.parseInt(graphRow[0]);
			String description = graphRow[3].replace("\"", "\\\"");
			String animate = graphRow[4];

			// Generates JSON response for graph
			var equationsJSON = new StringBuilder();

			// loops for every equation that is owned by the request graph
			for (String[] equation : db.selectWhere(
				SetupDB.GRAPH_EQUATIONS_TABLE,
				"ownerGraph=? ORDER BY zIndex",
				graphId
			)) {
				var zIndex = Integer.parseInt(equation[0]);
				String equationASCII = equation[2];
				var disabled = Boolean.parseBoolean(equation[3]);
				equationsJSON.append(",{");

				// Z Index
				equationsJSON.append("\"zIndex\":");
				equationsJSON.append(zIndex);
				equationsJSON.append(",");


				equationsJSON.append("\"equation\":");
				equationsJSON.append("\"");
				equationsJSON.append(equationASCII);
				equationsJSON.append("\",");
				equationsJSON.append("\"disabled\":");
				equationsJSON.append(disabled);
				equationsJSON.append("}");
			}


			var json = "{" +
				"\"name\":\"" +
				graphName +
				"\",\"description\":\"" +
				description +
				"\",\"animate\":" +
				animate +
				",\"equations\": [" +
				(equationsJSON.length() > 1 ? equationsJSON.substring(1) : "") +
				"]" +
				"}";

			byte[] response = json.getBytes();

			// adds json content-type
			ex.getResponseHeaders().add(
				"Content-Type",
				FileHandler.MIME_TYPES.get(".json")
			);
			ex.sendResponseHeaders(200, response.length);
			body.write(response);
			body.close();


		} catch (SQLException | NullPointerException | NumberFormatException e) {
			// Send internal server error
			ex.sendResponseHeaders(500, 0);
			body.close();
			e.printStackTrace();
		}
	}

	public void deleteGraph(RequestInfo req) throws IOException {
		HttpExchange ex = req.getEx();
		OutputStream body = req.getResponseBody();
		String sessionId = req.getSessionId();

		// Request Arguments
		Map<String, String> reqBody = req.getRequestBody();
		String graphName = reqBody.get("name");

		try {
			// check if permitted or not
			if (!auth.isAuthorizedForGraph(sessionId, graphName))
				throw new NoPermissionException();
		} catch (SQLException e) {
			// Send internal server error
			ex.sendResponseHeaders(500, 0);
			body.close();
			e.printStackTrace();
			return;
		} catch (NoPermissionException | IllegalArgumentException e) {
			// if not permitted send unauthorized error
			ex.sendResponseHeaders(401, 0);
			body.close();
			return;
		}

		// get owner ID (we know its valid because we checked authorization)
		Integer ownerId = auth.getSessionManager().getUID(sessionId);
		try (var db = new SQLDb(SetupDB.DB_NAME)) {
			db.deleteWhere(SetupDB.GRAPHS_TABLE,
				"owner=? AND graphName=?",
				ownerId,
				graphName
			);
			ex.sendResponseHeaders(200, 0);
			body.close();
		} catch (SQLException e) {
			// Send internal server error
			ex.sendResponseHeaders(500, 0);
			body.close();
			e.printStackTrace();
		}
	}

	public void createGraph(RequestInfo req) throws IOException {
		HttpExchange ex = req.getEx();
		OutputStream body = req.getResponseBody();
		String sessionId = req.getSessionId();
		byte[] response;

		// assure session exist
		if (!auth.getSessionManager().isValidToken(sessionId)) {
			// send not logged in error if session token is not valid
			response = "Not Logged In".getBytes();
			ex.sendResponseHeaders(401, response.length);
			body.write(response);
			body.close();
			return;
		}

		Integer uid = auth.getSessionManager().getUID(sessionId);
		String email;

		// gets email from UID
		try (var db = new SQLDb(SetupDB.DB_NAME)) {
			String[] user = db.selectWhere(SetupDB.USERS_TABLE, "id=?", uid)[0];
			email = user[1];
		} catch (SQLException e) {
			// Send internal server error
			ex.sendResponseHeaders(500, 0);
			body.close();
			e.printStackTrace();
			return;
		}


		// Request Arguments
		Map<String, String> reqBody = req.getRequestBody();

		if (!reqBody.containsKey("name")) {
			response = "Requires name argument".getBytes();
			ex.sendResponseHeaders(401, response.length);
			body.write(response);
			body.close();
			return;
		}

		String name = reqBody.get("name");

		// Ensure name is not duplicated
		try (var db = new SQLDb(SetupDB.DB_NAME)) {
			String[][] duplications = db.selectWhere(SetupDB.GRAPHS_TABLE,
				"owner=? AND graphName=?",
				uid,
				name
			);

			if (duplications.length > 0) {
				// Send internal server error if there are any duplications
				response = "Duplicate Graph".getBytes();
				ex.sendResponseHeaders(409, response.length);
				body.write(response);
				body.close();
				return;
			}
		} catch (SQLException e) {
			// Send internal server error
			ex.sendResponseHeaders(500, 0);
			body.close();
			e.printStackTrace();
			return;
		}


		// Generate a unique graph ID
		int graphId = 0;

		try (var db = new SQLDb(SetupDB.DB_NAME)) {
			String[][] duplicateGraphIds;
			var isGraphIdDuplicated = true;

			// repeat until generated id is not a duplicate
			while (isGraphIdDuplicated) {
				graphId = UUID.randomUUID().hashCode();

				// get all graphs in table with same id
				duplicateGraphIds = db.selectWhere(SetupDB.GRAPHS_TABLE, "id=?", graphId);

				// graph duplicated is true if there are any duplicates found
				isGraphIdDuplicated = duplicateGraphIds.length > 0;
			}
		} catch (SQLException e) {
			// Send internal server error
			ex.sendResponseHeaders(500, 0);
			body.close();
			e.printStackTrace();
			return;
		}

		try (var db = new SQLDb(SetupDB.DB_NAME)) {
			db.insert(
				SetupDB.GRAPHS_TABLE,
				graphId, // graph id
				uid,  // owner id
				name, // graph name
				format("%s's Graph", email), // graph description
				false // animate
			);
		} catch (SQLException e) {
			// Send internal server error
			ex.sendResponseHeaders(500, 0);
			body.close();
			e.printStackTrace();
			return;
		}

		ex.sendResponseHeaders(200, 0);
		body.close();
	}

	public void updateGraph(RequestInfo req) throws IOException {
		HttpExchange ex = req.getEx();
		String sessionId = req.getSessionId();
		Map<String, String> args = req.getRequestBody();
		OutputStream body = req.getResponseBody();

		// request args
		var equationsTxt = new ArrayList<String>();
		var equationsDisabled = new ArrayList<Boolean>();
		int graphId, equationNum, uid;
		String description;
		String graphName;

		try (var db = new SQLDb(SetupDB.DB_NAME)) {
			// parses request arguments
			graphName = Objects.requireNonNull(args.get("name"));
			description = Objects.requireNonNull(args.get("description"));

			// assures that user is authorized for change
			if (!auth.isAuthorizedForGraph(sessionId, graphName))
				throw new NoPermissionException();
			uid = auth.getSessionManager().getUID(sessionId);

			// get graph ID
			String result = db.selectWhere(
				SetupDB.GRAPHS_TABLE,
				"owner=? AND graphName=?",
				uid,
				graphName
			)[0][0];
			graphId = Integer.parseInt(result);

			// gets equation length
			equationNum = Integer.parseInt(args.get("equation_length"));

			// reads all equations from request
			for (int i = 0; i < equationNum; i++) {
				boolean disabled = Boolean.parseBoolean(args.get(format("%s_disabled", i)));
				String txt = Objects.requireNonNull(args.get(format("%s_equation", i)));
				equationsTxt.add(txt);
				equationsDisabled.add(disabled);
			}
		} catch (NullPointerException | NumberFormatException e) {
			// send error back to client
			var msg = "Invalid Request Parameters".getBytes();
			ex.sendResponseHeaders(400, msg.length);
			body.write(msg);
			body.close();
			return;
		} catch (SQLException e) {
			// Send internal server error
			ex.sendResponseHeaders(500, 0);
			body.close();
			e.printStackTrace();
			return;
		} catch (NoPermissionException | IllegalArgumentException e) {
			// Send internal server error
			ex.sendResponseHeaders(401, 0);
			body.close();
			e.printStackTrace();
			return;
		}

		try (var db = new SQLDb(SetupDB.DB_NAME)) {

			// clears all previous equations
			db.deleteWhere(
				SetupDB.GRAPH_EQUATIONS_TABLE,
				"ownerGraph=?",
				graphId
			);

			db.execPrepared(
				format(
					"UPDATE %s SET %s=? WHERE id=?",
					SetupDB.GRAPHS_TABLE.getName(),
					"description"
				),
				description,
				graphId
			);

			String equationTxt;
			boolean disabled;
			for (int z = 0; z < equationNum; z++) {
				equationTxt = equationsTxt.get(z);
				disabled = equationsDisabled.get(z);

				// adds equation
				db.insert(
					SetupDB.GRAPH_EQUATIONS_TABLE,
					z,
					graphId,
					equationTxt,
					disabled
				);
			}
		} catch (SQLException e) {
			// Send internal server error
			ex.sendResponseHeaders(500, 0);
			body.close();
			e.printStackTrace();
		}

		ex.sendResponseHeaders(200, 0);
		body.close();
	}
}
